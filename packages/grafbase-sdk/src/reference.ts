import { Enum } from './enum'
import { ListDefinition } from './field/list'
import { Type } from './type'

export class ReferenceDefinition {
  referencedType: string
  isOptional: boolean

  constructor(referencedType: Type | Enum) {
    this.referencedType = referencedType.name
    this.isOptional = false
  }

  public optional(): ReferenceDefinition {
    this.isOptional = true

    return this
  }

  public list(): ListDefinition {
    return new ListDefinition(this)
  }

  public toString(): string {
    const required = this.isOptional ? '' : '!'

    return `${this.referencedType}${required}`
  }
}
