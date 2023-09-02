/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Serializer, scalarEnum } from '@metaplex-foundation/umi/serializers';

export enum UseMethod {
  Burn,
  Multiple,
  Single,
}

export type UseMethodArgs = UseMethod;

export function getUseMethodSerializer(): Serializer<UseMethodArgs, UseMethod> {
  return scalarEnum<UseMethod>(UseMethod, {
    description: 'UseMethod',
  }) as Serializer<UseMethodArgs, UseMethod>;
}
