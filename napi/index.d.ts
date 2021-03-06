/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

/** The search mode. */
export const enum SearchMode {
  /**
   * Return the first response.
   *
   * For example, `["a", "b", "c"]` and `"c"` returns the fast,
   * we return `"c"`.
   *
   * This is the default mode.
   */
  FastFirst = 0,
  /**
   * Return according to the order of the response.
   *
   * For example, even if `["a", "b", "c"]` and `"c"` returns the fast,
   * we still wait for `"a"` and return `"a"`. If `"a"` has no result,
   * we return `"b"`.
   */
  OrderFirst = 1
}
/** [napi-rs] The metadata of the artist of a song. */
export interface Artist {
  /** The identifier of this artist. */
  id: string
  /** The name of this artist. */
  name: string
}
/** [napi-rs] The metadata of the album of a song. */
export interface Album {
  /** The identifier of this artist. */
  id: string
  /** The name of this album. */
  name: string
}
/** [napi-rs] The metadata of a song. */
export interface Song {
  /** The identifier of this song. */
  id: string
  /** The name of this song. */
  name: string
  /** The duration of this song. */
  duration?: number
  /** The artist of this song. */
  artists: Array<Artist>
  /** The album of this song. */
  album?: Album
  /**
   * The context of this song.
   *
   * For example, the URI identifier of this song.
   */
  context?: Record<string, string>
}
/** [napi-rs] The song identifier with the engine information. */
export interface SongSearchInformation {
  /** The retrieve source of this song, for example: `bilibili`. */
  source: string
  /** The serialized identifier of this song. */
  identifier: string
  /** The details of this song. */
  song?: Song
  /** The pre-retrieve result of this search. */
  preRetrieveResult?: RetrievedSongInfo
}
/** [napi-rs] The information of the song retrieved with `retrieve()`. */
export interface RetrievedSongInfo {
  /** The retrieve source of this song, for example: `bilibili`. */
  source: string
  /** The URL of this song. */
  url: string
}
/** [napi-rs] The context. */
export interface Context {
  /** The proxy URI */
  proxyUri?: string
  /** Whether to enable FLAC support. */
  enableFlac?: boolean
  /** The search mode for waiting the response. */
  searchMode?: SearchMode
  /** The config for engines. */
  config?: Record<string, string>
}
/** The available logging output. */
export const enum LoggingType {
  /**
   * Output to the console.
   *
   * Output all messages including 'trace' by default.
   * You can change this by setting the `RUST_LOG` environment variable.
   *
   * Available values are `error`, `warn`, `info`, `debug`, and `trace`.
   * For more information, see <https://docs.rs/log/latest/log/enum.LevelFilter.html#variants>
   */
  ConsoleEnv = 0
}
/**
 * Enable to log to the specified output.
 *
 * @see {@link LoggingType}
 */
export function enableLogging(logType: LoggingType): void
export type JsExecutor = Executor
export class Executor {
  constructor()
  list(): Array<string>
  search(engines: Array<string>, song: Song, ctx: Context): Promise<SongSearchInformation>
  retrieve(song: SongSearchInformation, ctx: Context): Promise<RetrievedSongInfo>
}
