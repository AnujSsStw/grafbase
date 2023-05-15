import { ListDefinition } from './field/list'
import {
  DefaultDefinition,
  LengthLimitedStringDefinition,
  ScalarDefinition,
  SearchDefinition,
  UniqueDefinition
} from './field/typedefs'
import { Model } from './model'
import { ReferenceDefinition } from './reference'
import { RelationDefinition } from './relation'
import { GrafbaseSchema } from './grafbase-schema'
import { Config, ConfigInput } from './config'

export type FieldShape =
  | ScalarDefinition
  | RelationDefinition
  | ListDefinition
  | SearchDefinition
  | ReferenceDefinition
  | UniqueDefinition
  | DefaultDefinition
  | LengthLimitedStringDefinition

export type AtLeastOne<T> = [T, ...T[]]
export type ScalarType = string | number | Date | object | boolean
export type EnumShape = AtLeastOne<string> | { [s: number]: string }
export type RelationRef = RelationF | Model
export type Searchable = ScalarDefinition | ListDefinition | UniqueDefinition

type RelationF = () => Model

export const g = new GrafbaseSchema()

export function config(input: ConfigInput): Config {
  return new Config(input)
}
