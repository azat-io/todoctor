<script lang="ts">
  import type { Options } from '@dicebear/open-peeps'

  import * as openPeeps from '@dicebear/open-peeps'
  import { createAvatar } from '@dicebear/core'

  export let data: string = ''

  openPeeps.schema.properties!.face

  let transformToCode = (string: string, length: number): number => {
    let hash = 0

    for (let i = 0, len = string.length; i < len; i++) {
      let chr = string.charCodeAt(i)
      hash = (hash << 5) - hash + chr
      hash |= 0
    }
    return Math.abs(hash) % length
  }

  let properties = openPeeps.schema.properties as {
    facialHair: { items: { enum: string[] } }
    face: { items: { enum: string[] } }
    head: { items: { enum: string[] } }
  }

  let faceItems = properties.face.items.enum as unknown as Options['face']
  let selectedFace = transformToCode(data, faceItems!.length)
  let face = [faceItems![selectedFace]]

  let headItems = properties.head.items.enum as unknown as Options['head']
  let selectedHead = transformToCode(data, headItems!.length)
  let head = [headItems![selectedHead]]

  let facialHairItems = properties.facialHair.items
    .enum as unknown as Options['facialHair']
  let selectedFacialHair = transformToCode(data, facialHairItems!.length)
  let facialHair = [facialHairItems![selectedFacialHair]]

  let skinColorItems = ['694d3d', 'ae5d29', 'd08b5b', 'edb98a', 'ffdbb4']
  let selectedSkinColor = transformToCode(data, skinColorItems.length)
  let skinColor = [skinColorItems[selectedSkinColor]]

  let avatar = createAvatar(openPeeps, {
    backgroundColor: ['a6e8b3'],
    facialHairProbability: 0,
    randomizeIds: false,
    facialHair,
    scale: 100,
    radius: 50,
    skinColor,
    size: 42,
    face,
    head,
  }).toDataUri()
</script>

<img src={avatar} alt="Avatar" />
