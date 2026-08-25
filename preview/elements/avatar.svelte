<script lang="ts">
  import definition from '@dicebear/styles/open-peeps.json'
  import { Avatar, Style } from '@dicebear/core'

  export let data: string = ''

  function transformToCode(string: string, length: number): number {
    if (length <= 0) {
      return 0
    }

    let hash = 0

    for (let stringLength = string.length, i = 0; i < stringLength; i++) {
      let chr = string.charCodeAt(i)
      hash = (hash << 5) - hash + chr
      hash |= 0
    }
    return Math.abs(hash) % length
  }

  function pickOption<T>(items: T[], seed: string): T[] {
    let value = items[transformToCode(seed, items.length)]
    return value === undefined ? [] : [value]
  }

  let style = new Style(definition)

  let expressionItems = Object.keys(
    definition.components.expression.variants,
  ) as (keyof typeof definition.components.expression.variants)[]
  let headItems = Object.keys(
    definition.components.head.variants,
  ) as (keyof typeof definition.components.head.variants)[]
  let facialHairItems = Object.keys(
    definition.components.facialHair.variants,
  ) as (keyof typeof definition.components.facialHair.variants)[]

  let skinColorItems = ['694d3d', 'ae5d29', 'd08b5b', 'edb98a', 'ffdbb4']

  let avatarImage = new Avatar(style, {
    facialHairVariant: pickOption(facialHairItems, data),
    expressionVariant: pickOption(expressionItems, data),
    skinColor: pickOption(skinColorItems, data),
    headVariant: pickOption(headItems, data),
    backgroundColor: ['a6e8b3'],
    facialHairProbability: 0,
    borderRadius: 50,
    size: 42,
  })

  let avatar = avatarImage.toDataUri()
</script>

<img
  src={avatar}
  alt="Avatar"
/>
