const path = require("path");
const {
  Kinobi,
  RenderJavaScriptVisitor,
  SetInstructionAccountDefaultValuesVisitor,
  SetLeafWrappersVisitor,
  RenameNodesVisitor,
  SetInstructionBytesCreatedOnChainVisitor,
  DeleteNodesVisitor,
  UnwrapStructVisitor,
  UnwrapDefinedTypesVisitor,
  TransformNodesVisitor,
  assertInstructionNode,
  InstructionNode,
} = require("@lorisleiva/kinobi");

// Paths.
const clientDir = path.join(__dirname, "..", "clients");
const idlDir = path.join(__dirname, "..", "idls");

// Instanciate Kinobi.
const kinobi = new Kinobi([
  path.join(idlDir, "mpl_token_auth_rules.json"),
  path.join(idlDir, "mpl_token_metadata.json"),
]);

// Rename nodes.
kinobi.update(
  new RenameNodesVisitor({
    mplTokenAuthRules: {
      prefix: "Ta",
      accounts: {
        FrequencyAccount: "RuleSetFrequency",
      },
      instructions: {
        CreateOrUpdate: "CreateOrUpdateRuleSet",
        Validate: "ValidateRuleSet",
        WriteToBuffer: "WriteRuleSetToBuffer",
      },
      types: {
        Key: "TokenAuthRulesKey",
        CreateOrUpdateArgs: "CreateOrUpdateRuleSetArgs",
        ValidateArgs: "ValidateRuleSetArgs",
        WriteToBufferArgs: "WriteRuleSetToBufferArgs",
      },
    },
    mplTokenMetadata: {
      prefix: "Tm",
      types: { Key: "TokenMetadataKey" },
      accounts: { MasterEditionV2: "MasterEdition" },
    },
  })
);

// Remove nodes.
kinobi.update(
  new DeleteNodesVisitor([
    // Duplicated from token auth rules.
    { type: "definedType", name: "Payload", program: "mplTokenMetadata" },
    { type: "definedType", name: "PayloadType", program: "mplTokenMetadata" },
    // Deprecated nodes.
    { type: "account", name: "ReservationListV1", program: "mplTokenMetadata" },
    { type: "account", name: "ReservationListV2", program: "mplTokenMetadata" },
    { type: "account", name: "MasterEditionV1", program: "mplTokenMetadata" },
  ])
);

// Wrap leaves.
kinobi.update(
  new SetLeafWrappersVisitor({
    // "splSystem.CreateAccount.lamports": { kind: "SolAmount" },
  })
);

// Set default values for instruction accounts.
kinobi.update(
  new SetInstructionAccountDefaultValuesVisitor([
    // { instruction: "TransferSol", account: "source", kind: "identity" },
  ])
);

// Set instruction bytes created on chain.
kinobi.update(
  new SetInstructionBytesCreatedOnChainVisitor({
    // CreateAccount: { kind: "arg", name: "space" },
  })
);

// Unwrap data attribute of Metadata account.
kinobi.update(new UnwrapDefinedTypesVisitor(["Data"]));
kinobi.update(
  new UnwrapStructVisitor({
    "mplTokenMetadata.Metadata": ["data"],
    "mplTokenMetadata.CreateMetadataAccountInstructionArgs": ["data"],
  })
);

// Mark mint account has optional signer on Create instruction.
kinobi.update(
  new TransformNodesVisitor([
    {
      selector: { type: "instruction", name: "Create" },
      transformer: (node) => {
        assertInstructionNode(node);
        const newAccounts = node.accounts.map((account) => {
          return account.name === "mint"
            ? { ...account, isOptionalSigner: true }
            : account;
        });
        return new InstructionNode(node.metadata, newAccounts, node.args);
      },
    },
  ])
);

// Render JavaScript.
const jsDir = path.join(clientDir, "js", "src", "generated");
const prettier = require(path.join(clientDir, "js", ".prettierrc.json"));
kinobi.accept(new RenderJavaScriptVisitor(jsDir, { prettier }));
