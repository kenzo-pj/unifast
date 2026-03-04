# Sanitization Corpus

## Safe HTML (should be preserved when allowed)

<div class="note">
  <p>This is a safe div.</p>
</div>

<em>Emphasis via HTML</em>

<strong>Strong via HTML</strong>

## Dangerous HTML (should be stripped)

<script>alert('xss')</script>

<iframe src="https://evil.com"></iframe>

<img src="x" onerror="alert('xss')">

<a href="javascript:alert('xss')">Evil link</a>

<style>body { display: none; }</style>

## Event Handlers (should be stripped)

<div onclick="alert('click')">Click trap</div>

<input onfocus="alert('focus')">

## Data URLs (potentially dangerous)

<a href="data:text/html,<script>alert('xss')</script>">Data URL</a>

## Safe Content

This paragraph is **safe** and should always render.

- Safe list item
- Another safe item

[Safe link](https://example.com)
