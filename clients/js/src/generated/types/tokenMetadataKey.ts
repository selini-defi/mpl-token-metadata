/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Context, Serializer } from '@lorisleiva/js-core';

export enum TokenMetadataKey {
  Uninitialized,
  EditionV1,
  MasterEditionV1,
  ReservationListV1,
  MetadataV1,
  ReservationListV2,
  MasterEditionV2,
  EditionMarker,
  UseAuthorityRecord,
  CollectionAuthorityRecord,
  TokenOwnedEscrow,
  TokenRecord,
  MetadataDelegate,
}

export function getTokenMetadataKeySerializer(
  context: Pick<Context, 'serializer'>
): Serializer<TokenMetadataKey> {
  const s = context.serializer;
  return s.enum<TokenMetadataKey>(TokenMetadataKey, 'TokenMetadataKey');
}