# Output Capture Implementation Notes

## Current Status (v0.1.0)

The current MVP implementation **does not capture command output**. Shelltape records:

✅ Command text
✅ Exit code
✅ Execution timing
✅ Working directory
✅ Session information
✅ Shell environment

❌ Command output (stdout/stderr)

## Why Output Capture is Complex

Capturing command output without breaking the user experience is challenging because:

1. **PTY Requirements**: Commands need to be wrapped in a pseudo-terminal to capture output while preserving interactive features (colors, cursor movements, etc.)

2. **Shell Integration Limitations**: Simple shell hooks can't intercept stdout/stderr without affecting the user's terminal experience

3. **Performance**: Capturing and storing large outputs can slow down the shell

## Planned Approaches

### Option A: PTY Wrapper (Preferred)

**Pros:**
- Transparent capture of all output
- Preserves terminal features (colors, cursor positioning)
- Works with interactive commands

**Cons:**
- More complex implementation
- Requires careful handling of terminal state

**Implementation:**
```rust
// Wrap command execution in a PTY
// Similar to how `script` command works
```

### Option B: Script Command Integration

**Pros:**
- Leverages existing `script` utility
- Simple implementation

**Cons:**
- Requires external dependency
- May not be available on all systems
- Performance overhead

**Implementation:**
```bash
# In shell hooks
script -q -c "$command" /dev/null | tee "$SHELLTAPE_OUTPUT_FILE"
```

### Option C: Shell-Specific Redirection

**Pros:**
- No external dependencies
- Shell-native approach

**Cons:**
- Different implementation for each shell
- Can break interactive commands
- May interfere with user's own redirections

**Implementation (Bash):**
```bash
# Would need careful exec redirection
exec 3>&1 4>&2
exec 1> >(tee -a "$SHELLTAPE_OUTPUT_FILE")
exec 2>&1
```

## Recommended Implementation Plan

### Phase 4.1: Output Capture

1. **Add PTY wrapper crate dependency**
   ```toml
   [dependencies]
   portable-pty = "0.8"
   ```

2. **Implement PTY capture module** (`src/pty_capture.rs`)
   - Wrap command execution
   - Capture stdout/stderr
   - Preserve terminal features

3. **Update shell hooks**
   - Detect if shelltape has PTY support
   - Use PTY wrapper when available
   - Fallback to no-capture mode

4. **Add configuration options**
   ```toml
   [recording]
   capture_output = true
   max_output_size = 100_000  # bytes
   capture_stderr = true
   ```

5. **Update TUI to display output**
   - Add output panel in detail view
   - Syntax highlighting for common formats
   - Pagination for large outputs

### Phase 4.2: Optimization

1. **Async output writing** - Don't block shell on I/O
2. **Output compression** - Store large outputs compressed
3. **Smart truncation** - Truncate at logical boundaries (lines)
4. **Selective capture** - Only capture output for specific patterns

## Timeline

- **v0.2.0**: PTY wrapper implementation
- **v0.3.0**: Configuration and optimization
- **v0.4.0**: Advanced features (compression, filtering)

## Workarounds for Current Version

Users who need output capture now can:

1. **Manually pipe to tee:**
   ```bash
   command | tee output.log
   ```

2. **Use script command:**
   ```bash
   script -c "command" output.log
   ```

3. **Review terminal scrollback** - Output is still visible in terminal

## References

- [portable-pty crate](https://docs.rs/portable-pty/)
- [How `script` command works](https://man7.org/linux/man-pages/man1/script.1.html)
- [Atuin issue on output capture](https://github.com/atuinsh/atuin/issues/68)
- [Terminal handling in Rust](https://www.joshmcguigan.com/blog/build-your-own-shell-rust/)

## Contributing

If you're interested in implementing output capture, please:

1. Review this document
2. Check existing issues/PRs
3. Discuss approach on GitHub before major work
4. Consider starting with Option B (script integration) as a proof of concept

---

**Note:** This is a living document and will be updated as implementation progresses.
