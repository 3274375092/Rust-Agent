# Rust Agent

Rust Agent 鏄竴涓湰鍦板懡浠よ AI 鍔╂墜銆備綘鍙互鎶婂畠鎺ュ埌 DeepSeek銆丱penAI銆丱penRouter銆丱llama銆丩M Studio 绛夊吋瀹?OpenAI Chat Completions 鐨勬ā鍨嬫湇鍔★紝鐒跺悗璁╁畠鍦ㄤ綘鐨勯」鐩洰褰曢噷璇绘枃浠躲€佹敼鏂囦欢銆佹墽琛屽彈闄愮殑妫€鏌ュ懡浠ゃ€?
## 浣犲彲浠ョ敤瀹冨仛浠€涔?
- 鍜屾ā鍨嬭繘琛屽杞璇濄€?- 璁╂ā鍨嬫煡鐪嬮」鐩枃浠跺垪琛ㄣ€?- 璁╂ā鍨嬫寜琛岃鍙栨寚瀹氭枃浠跺唴瀹广€?- 璁╂ā鍨嬩慨鏀规枃浠跺唴瀹广€?- 璁╂ā鍨嬫墽琛屽畨鍏ㄧ櫧鍚嶅崟閲岀殑鍛戒护锛屼緥濡?`cargo check`銆乣git diff`銆?
> 濡傛灉浣犲笇鏈涙ā鍨嬭兘璇绘枃浠躲€佹敼鏂囦欢鎴栨墽琛屽懡浠わ紝璇蜂娇鐢ㄦ敮鎸?tool calling / function calling 鐨勬ā鍨嬨€?
## 蹇€熷紑濮?
### 1. 鍑嗗 `.env`

澶嶅埗绀轰緥閰嶇疆鏂囦欢锛?
```powershell
Copy-Item .env.example .env
```

鐒跺悗缂栬緫 `.env`锛?
```dotenv
API_KEY=sk-your-api-key
BASE_URL=https://api.deepseek.com
MODEL=deepseek-v4-pro
```

甯哥敤閰嶇疆椤癸細

- `API_KEY`锛氫綘鐨勬ā鍨嬫湇鍔?API Key銆?- `BASE_URL`锛氭ā鍨嬫湇鍔″湴鍧€銆?- `MODEL`锛氳浣跨敤鐨勬ā鍨嬪悕绉般€?
`.env` 閲岄€氬父鍖呭惈瀵嗛挜锛屼笉瑕佹妸瀹冩彁浜ゅ埌 Git 浠撳簱銆?
### 2. 杩愯绋嬪簭

濡傛灉浣犳槸浠庢簮鐮佽繍琛岋細

```powershell
cargo run
```

濡傛灉浣犲凡缁忔湁缂栬瘧濂界殑绋嬪簭锛?
```powershell
.\rig-agent.exe
```

鍚姩鍚庣湅鍒版彁绀虹锛?
```text
You>
```

鐩存帴杈撳叆浣犵殑闂鍗冲彲銆傞€€鍑鸿杈撳叆锛?
```text
exit
```

鎴栵細

```text
quit
```

## `.env` 搴旇鏀惧湪鍝噷

`.env` 涓嶄細琚紪璇戣繘 `.exe`锛屽畠鏄湪绋嬪簭鍚姩鏃惰鍙栫殑銆?
褰撳墠绋嬪簭浣跨敤 `dotenvy::dotenv().ok()` 鍔犺浇閰嶇疆锛屾墍浠ュ畠浼氫粠鈥滃惎鍔ㄧ▼搴忔椂鐨勫綋鍓嶅伐浣滅洰褰曗€濇煡鎵?`.env`銆?
寮€鍙戞椂閫氬父杩欐牱鏀撅細

```text
E:\Rust\Rust-Agent\
鈹溾攢鈹€ .env
鈹溾攢鈹€ Cargo.toml
鈹斺攢鈹€ src\
```

鐒跺悗鍦ㄩ」鐩牴鐩綍杩愯锛?
```powershell
cd E:\Rust\Rust-Agent
cargo run
```

鍙戝竷缁欑敤鎴锋椂锛屾帹鑽愭妸 `.env` 鍜?`.exe` 鏀惧湪鍚屼竴涓洰褰曪細

```text
my-agent\
鈹溾攢鈹€ rig-agent.exe
鈹斺攢鈹€ .env
```

鐒跺悗浠庤繖涓洰褰曞惎鍔細

```powershell
cd my-agent
.\rig-agent.exe
```

杩欐牱绋嬪簭浼氳鍙栵細

```text
my-agent\.env
```

娉ㄦ剰锛氬鏋滀綘浠庡叾浠栫洰褰曞惎鍔ㄨ繖涓?exe锛岀▼搴忎細浼樺厛璇诲彇鈥滃惎鍔ㄧ洰褰曗€濅笅鐨?`.env`锛屼笉鏄?exe 鎵€鍦ㄧ洰褰曘€?
## 甯歌妯″瀷閰嶇疆

### DeepSeek

```dotenv
API_KEY=sk-your-deepseek-key
BASE_URL=https://api.deepseek.com
MODEL=deepseek-v4-pro
```

### OpenAI

```dotenv
API_KEY=sk-your-openai-key
BASE_URL=https://api.openai.com/v1
MODEL=gpt-4.1
```

### OpenRouter

```dotenv
API_KEY=sk-or-your-openrouter-key
BASE_URL=https://openrouter.ai/api/v1
MODEL=deepseek/deepseek-chat-v3.1
```

### Ollama 鏈湴妯″瀷

```dotenv
API_KEY=ollama
BASE_URL=http://localhost:11434/v1
MODEL=qwen2.5-coder:7b
```

### LM Studio 鏈湴妯″瀷

```dotenv
API_KEY=lm-studio
BASE_URL=http://localhost:1234/v1
MODEL=local-model
```

## 鍐呯疆宸ュ叿璇存槑

### 鏂囦欢鍒楄〃

`list_files` 浼氬垪鍑虹洰褰曚笅鐨勬枃浠跺拰鏂囦欢澶癸紝骞堕粯璁よ烦杩囪繖浜涘唴瀹癸細

- `.git`
- `target`
- `node_modules`
- `.env`
- `Cargo.lock`

### 鏂囦欢璇诲彇

`read_file` 鍙互璇诲彇鏁翠釜鏂囦欢锛屼篃鍙互鍙鍙栨寚瀹氳鑼冨洿锛屽噺灏戞ā鍨嬩笂涓嬫枃娑堣€椼€?
### 鏂囦欢缂栬緫

`edit_file` 鍙互鏇挎崲鏁翠釜鏂囦欢锛屾垨鏇挎崲鏂囦欢涓涓€娆″嚭鐜扮殑鎸囧畾鍐呭銆傚缓璁瘡娆＄紪杈戝悗閫氳繃 `git diff` 妫€鏌ョ粨鏋溿€?
### 鍛戒护鎵ц

`run_command` 鍙兘鎵ц鐧藉悕鍗曢噷鐨勫懡浠わ紝涓嶄細寮€鏀句换鎰?shell 鍛戒护銆?
鍏佽鐨勫懡浠ゅ寘鎷細

- `cargo check`
- `cargo test`
- `cargo clippy`
- `cargo fmt`
- `cargo build`
- `git status`
- `git diff`
- `git log`
- `git show`
- `rustc --version`

鍛戒护鏈€澶氳繍琛?60 绉掞紝缁撴灉浼氳繑鍥?`stdout`銆乣stderr`銆乣exit_code` 鍜?`success`銆?
## 瀹夊叏鎻愰啋

- 涓嶈鍏紑 `.env`锛岄噷闈㈠彲鑳芥湁 API Key銆?- 璁╂ā鍨嬩慨鏀规枃浠跺墠锛屽缓璁厛鎻愪氦鎴栧浠藉綋鍓嶅伐浣滃尯銆?- 璁╂ā鍨嬩慨鏀规枃浠跺悗锛屽缓璁娇鐢?`git diff` 妫€鏌ユ敼鍔ㄣ€?- 濡傛灉妯″瀷涓嶈兘璋冪敤宸ュ叿锛岃纭浣犱娇鐢ㄧ殑妯″瀷鏀寔 function calling銆?
## 浠庢簮鐮佹瀯寤?
濡傛灉浣犳兂鑷繁缂栬瘧锛?
```powershell
cargo build --release
```

缂栬瘧鍚庣殑绋嬪簭閫氬父浣嶄簬锛?
```text
target\release\rig-agent.exe
```

鍙互鎶婂畠澶嶅埗鍒板崟鐙洰褰曪紝骞跺湪鍚岀洰褰曞噯澶?`.env` 鍚庤繍琛屻€
