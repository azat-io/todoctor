import type { PluginOptionsByType, TooltipItem, ChartType } from 'chart.js'

type ChartPlugins<T extends ChartType> = Partial<PluginOptionsByType<T>>

// Tooltip and legend options are the same for every chart type.
// Only the color of the tooltip swatch differs, so each chart passes its own.
// Colors are read on every call, which keeps the tooltip on the active theme.
export function createChartPlugins<T extends ChartType>(
  computedStyles: CSSStyleDeclaration | undefined,
  getLabelBackgroundColor: (item: TooltipItem<T>) => undefined | string,
): ChartPlugins<T> {
  function readProperty(name: string): undefined | string {
    return computedStyles?.getPropertyValue(name)
  }

  return {
    tooltip: {
      callbacks: {
        labelColor: (item: TooltipItem<T>) => ({
          borderColor: readProperty('--color-border-primary'),
          backgroundColor: getLabelBackgroundColor(item),
          borderRadius: 2,
          borderWidth: 0,
        }),
      },
      titleFont: {
        family: readProperty('--font-family-base'),
        size: 16,
      },
      bodyFont: {
        family: readProperty('--font-family-base'),
        size: 16,
      },
      footerFont: {
        family: readProperty('--font-family-base'),
      },
      backgroundColor: readProperty('--color-background-secondary'),
      borderColor: readProperty('--color-border-primary'),
      titleColor: readProperty('--color-content-primary'),
      bodyColor: readProperty('--color-content-primary'),
      borderWidth: 1,
    },
    legend: {
      display: false,
    },
  } as ChartPlugins<T>
}
