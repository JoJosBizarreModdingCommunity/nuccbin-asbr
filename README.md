# nuccbin
A tool to serialize/deserialize several nuccChunkBinary .xfbin's from JoJo's Bizzarre Adventure: All Star Battle R

## Usage
- Get the latest version from [releases](https://github.com/JoJosBizarreModdingCommunity/nuccbin-asbr/releases).
- Drag and drop any of the .xfbin files in the list onto the nuccbin-asbr.exe.
- Add the changes to your .json file(s) by adding, removing, or editing entries.
- Apply your changes to the .xfbin file by dragging and dropping the newly created folder onto nuccbin-asbr.exe.

## Formats
nuccbin supports a number of in game nuccChunkBinary param / bin formats:
| File | Serialize | Deserialize | Extension |
| --- | --- | --- | --- |
| [anmofs](https://github.com/JoJosBizarreModdingCommunity/nuccbin-asbr/blob/main/src/nucc_binary/anm_offset.rs) | ✔️ | ✔️ | `json` |
| [CharViewerParam](https://github.com/JoJosBizarreModdingCommunity/nuccbin-asbr/blob/main/src/nucc_binary/char_viewer_param.rs) | ✔️ | ✔️ | `json` |
| [Characode](https://github.com/JoJosBizarreModdingCommunity/nuccbin-asbr/blob/main/src/nucc_binary/characode.rs) | ✔️ | ✔️  | `json` |
| [CommandListParam](https://github.com/JoJosBizarreModdingCommunity/nuccbin-asbr/blob/main/src/nucc_binary/command_list_param.rs) | ✔️ | ✔️ | `json` |
| [CustomCardParam](https://github.com/JoJosBizarreModdingCommunity/nuccbin-asbr/blob/main/src/nucc_binary/custom_card_param.rs) | ✔️ | ✔️ | `json` |
| [CustomizeDefaultParam](https://github.com/JoJosBizarreModdingCommunity/nuccbin-asbr/blob/main/src/nucc_binary/customize_default_param.rs) | ✔️ | ✔️ | `json` |
| [dds](https://github.com/JoJosBizarreModdingCommunity/nuccbin-asbr/blob/main/src/nucc_binary/dds.rs) | ✔️ | ✔️ |  `dds` |
| [DictionaryParam](https://github.com/JoJosBizarreModdingCommunity/nuccbin-asbr/blob/main/src/nucc_binary/dictionary_param.rs) | ✔️ | ✔️ |  `dds` |
| [DlcInfoParam](https://github.com/JoJosBizarreModdingCommunity/nuccbin-asbr/blob/main/src/nucc_binary/dlc_info_param.rs) | ✔️ | ✔️ | `json` |
| [effectprm](https://github.com/JoJosBizarreModdingCommunity/nuccbin-asbr/blob/main/src/nucc_binary/effectprm.rs) | ✔️ | ✔️ | `json` | 
| [ev](https://github.com/JoJosBizarreModdingCommunity/nuccbin-asbr/blob/main/src/nucc_binary/ev.rs) | ✔️ | ✔️ | `json` | 
| [lua](https://github.com/JoJosBizarreModdingCommunity/nuccbin-asbr/blob/main/src/nucc_binary/lua.rs) | ✔️ | ✔️ | `lua` | 
| [messageInfo](https://github.com/JoJosBizarreModdingCommunity/nuccbin-asbr/blob/main/src/nucc_binary/message_info.rs) | ✔️ | ✔️ | `json` | 
| [PlayerColorParam](https://github.com/JoJosBizarreModdingCommunity/nuccbin-asbr/blob/main/src/nucc_binary/player_color_param.rs) | ✔️ | ✔️ | `json` | 
| [prmload](https://github.com/JoJosBizarreModdingCommunity/nuccbin-asbr/blob/main/src/nucc_binary/prm_load.rs) | ✔️ | ✔️ | `json` |
| [SoundTestParam](https://github.com/JoJosBizarreModdingCommunity/nuccbin-asbr/blob/main/src/nucc_binary/sound_test_param.rs) | ✔️ | ✔️ | `json` | 
| [SpeakingLineParam](https://github.com/JoJosBizarreModdingCommunity/nuccbin-asbr/blob/main/src/nucc_binary/speaking_line_param.rs) | ✔️ | ✔️ | `json` | 
| [SupportCharaParam](https://github.com/JoJosBizarreModdingCommunity/nuccbin-asbr/blob/main/src/nucc_binary/support_chara_param.rs) | ✔️ | ✔️ | `json` |
| [xml](https://github.com/JoJosBizarreModdingCommunity/nuccbin-asbr/blob/main/src/nucc_binary/xml.rs) | ✔️ | ✔️ | `xml` | 


## Credits
This project is based on the [initial work](https://github.com/SutandoTsukai181/xfbin-nucc-binary) by SutandoTsukai181 on the original nuccChunkBinary parser for the All Star Battle R series.

Special thanks goes to:
* [HydraBladeZ](https://github.com/Al-Hydra) for reversing some formats.
* [SutandoTsukai181](https://github.com/SutandoTsukai181) for his initial work on the tool.
