autocmd BufWritePre * lua vim.lsp.buf.formatting_sync(nil, 300)
