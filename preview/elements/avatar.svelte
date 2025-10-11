<script lang="ts">
  import type { Options } from '@dicebear/open-peeps'

  import * as openPeeps from '@dicebear/open-peeps'
  import { createAvatar } from '@dicebear/core'

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

  function isString(value: unknown): value is string {
    return typeof value === 'string'
  }

  function pickOption<T extends string>(items: T[], seed: string): T[] {
    if (items.length === 0) {
      return []
    }

    let index = transformToCode(seed, items.length)
    let value = items[index]

    return value === undefined ? [] : [value]
  }

  type OptionArray<K extends keyof Options> = Extract<
    NonNullable<Options[K]>,
    string[]
  >

  let properties = openPeeps.schema.properties as {
    facialHair: { items: { enum?: unknown[] } }
    face: { items: { enum?: unknown[] } }
    head: { items: { enum?: unknown[] } }
  }

  let faceItems = (properties.face.items.enum ?? []).filter(isString)
  let headItems = (properties.head.items.enum ?? []).filter(isString)
  let facialHairItems = (properties.facialHair.items.enum ?? []).filter(
    isString,
  )

  let faceOptions = pickOption(faceItems, data) as OptionArray<'face'>
  let headOptions = pickOption(headItems, data) as OptionArray<'head'>
  let facialHairOptions = pickOption(
    facialHairItems,
    data,
  ) as OptionArray<'facialHair'>

  let skinColorItems = ['694d3d', 'ae5d29', 'd08b5b', 'edb98a', 'ffdbb4']
  let skinColor: string[] = []
  if (skinColorItems.length > 0) {
    let index = transformToCode(data, skinColorItems.length)
    let [fallback] = skinColorItems
    let value = skinColorItems[index] ?? fallback
    if (value !== undefined) {
      skinColor = [value]
    }
  }

  let avatar = createAvatar(openPeeps, {
    facialHair: facialHairOptions.length > 0 ? facialHairOptions : undefined,
    face: faceOptions.length > 0 ? faceOptions : undefined,
    head: headOptions.length > 0 ? headOptions : undefined,
    backgroundColor: ['a6e8b3'],
    facialHairProbability: 0,
    randomizeIds: false,
    scale: 100,
    radius: 50,
    skinColor,
    size: 42,
  }).toDataUri()
</script>

<img src={avatar} alt="Avatar" />
