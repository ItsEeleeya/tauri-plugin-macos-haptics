/*
 * Bindings for tauri_plugin_macos_haptics
 * https://github.com/ItsEeleeya/tauri-plugin-macos-haptics/
 */
import { invoke } from "@tauri-apps/api/core";

const CMD_PERFORM = "plugin:macos-haptics|perform";
const CMD_IS_SUPPORTED = "plugin:macos-haptics|is_supported";

let pluginSupported: boolean | null = null;

/**
 * Represents the different types of haptic feedback patterns.
 * @see https://developer.apple.com/documentation/appkit/nshapticfeedbackpattern
 */
export enum HapticFeedbackPattern {
  Alignment = 0,
  LevelChange = 1,
  Generic = 2,
}

/**
 * Specifies when the haptic feedback should be performed.
 * @see https://developer.apple.com/documentation/appkit/nshapticfeedbackmanager/performancetime
 */
export enum PerformanceTime {
  /** The system chooses the most appropriate time for feedback. */
  Default = 0,
  /** Provide immediate haptic feedback. */
  Now = 1,
  /** Provide haptic feedback after the next screen update. */
  DrawCompleted = 2,
}

/**
 * Checks if haptic feedback is supported on the current device.
 * Only supported on macOS 10.11 (OS X El Capitan) and above
 * @returns A promise that resolves to a boolean indicating support.
 */
export async function isSupported(): Promise<boolean> {
  if (pluginSupported === null) {
    pluginSupported = await invoke<boolean>(CMD_IS_SUPPORTED).catch(
      (_) => false
    );
  }
  return pluginSupported;
}

/**
 * Performs haptic feedback with the specified pattern and timing.
 *
 * IMPORTANT!:
 *  Call this method only in response to user-initiated actions.
 *  Ideally, visual feedback, such as a highlight or appearance of an alignment guide, should accompany the feedback.
 *
 * @example
 * ```typescript
 * import { isSupported, perform, HapticFeedbackPattern, PerformanceTime } from 'tauri-plugin-macos-haptics-api';
 *
 * isSupported().then((supported) => {
 *  if (supported) {
 *    perform(HapticFeedbackPattern.Generic, PerformanceTime.Now);
 *  }
 * });
 *
 * ```
 *
 * @param pattern The haptic feedback pattern to use.
 * @param performanceTime When to perform the haptic feedback.
 * @see https://developer.apple.com/documentation/appkit/nshapticfeedbackperformer/1441738-perform
 */
export async function perform(
  pattern: HapticFeedbackPattern = HapticFeedbackPattern.Generic,
  performanceTime: PerformanceTime = PerformanceTime.Default
): Promise<void> {
  await invoke<void>(CMD_PERFORM, {
    pattern,
    performanceTime,
  }).catch((error) =>
    console.error("Error performing haptic feedback: ", error)
  );
}
