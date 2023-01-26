import {
  generateSigner,
  none,
  percentAmount,
  publicKey,
  some,
  transactionBuilder,
} from '@lorisleiva/js-test';
import { fetchMint, Mint } from '@lorisleiva/mpl-essentials';
import test from 'ava';
import {
  createProgrammableNft,
  fetchMasterEdition,
  fetchMetadata,
  findMasterEditionPda,
  findMetadataPda,
  MasterEdition,
  Metadata,
  programmableConfig,
  TokenStandard,
} from '../src';
import { createMetaplex } from './_setup';

test('it can create a new ProgrammableNonFungible', async (t) => {
  // Given a new mint Signer.
  const mx = await createMetaplex();
  const mint = generateSigner(mx);

  // When we create a new ProgrammableNonFungible at this address.
  await transactionBuilder(mx)
    .add(
      createProgrammableNft(mx, {
        mint,
        name: 'My Programmable NFT',
        uri: 'https://example.com/my-programmable-nft.json',
        sellerFeeBasisPoints: percentAmount(5.5),
      })
    )
    .sendAndConfirm();

  // Then a Mint account was created with zero supply.
  const mintAccount = await fetchMint(mx, mint.publicKey);
  const masterEdition = findMasterEditionPda(mx, { mint: mint.publicKey });
  t.like(mintAccount, <Mint>{
    publicKey: publicKey(mint),
    supply: 0n,
    decimals: 0,
    mintAuthority: some(publicKey(masterEdition)),
    freezeAuthority: some(publicKey(masterEdition)),
  });

  // And a Metadata account was created.
  const metadata = findMetadataPda(mx, { mint: mint.publicKey });
  const metadataAccount = await fetchMetadata(mx, metadata);
  t.like(metadataAccount, <Metadata>{
    publicKey: publicKey(metadata),
    updateAuthority: publicKey(mx.identity),
    mint: publicKey(mint),
    tokenStandard: some(TokenStandard.ProgrammableNonFungible),
    name: 'My Programmable NFT',
    uri: 'https://example.com/my-programmable-nft.json',
    sellerFeeBasisPoints: 550,
    primarySaleHappened: false,
    isMutable: true,
    creators: some([
      { address: publicKey(mx.identity), verified: true, share: 100 },
    ]),
    collection: none(),
    uses: none(),
    collectionDetails: none(),
    programmableConfig: some(programmableConfig('V1', { ruleSet: none() })),
  });

  // And a MasterEdition account was created.
  const masterEditionAccount = await fetchMasterEdition(mx, masterEdition);
  t.like(masterEditionAccount, <MasterEdition>{
    publicKey: publicKey(masterEdition),
    supply: 0n,
    maxSupply: some(0n),
  });
});