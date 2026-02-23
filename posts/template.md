---
title: Template post
description: A super secret template post.
date: 2026-01-27
tags:
  - _no-index
  - test_tag
header_image: images/post.png
---

This is a top-secret template post. There's text in it. Isn't that cool?
Anything before the first double newline will get hidden behind the 'read more'
link. Oh, make sure to add the `_no-index` tag to this post!

---

Standard text!
*Italic text!*
**Bold text!**
***Bold italic text!***
~~Strikethrough.~~

It's a\
test here.

## Headings

# **H1 - bombsquad.dev**
# H1 - bombsquad.dev
## **H2 - bombsquad.dev**
## H2 - bombsquad.dev
### **H3 - bombsquad.dev**
### H3 - bombsquad.dev
#### **H4 - bombsquad.dev**
#### H4 - bombsquad.dev
##### **H5 - bombsquad.dev**
##### H5 - bombsquad.dev
###### **H6 - bombsquad.dev**
###### H6 - bombsquad.dev


## Code blocks

Inline code: `let x = 1 + 2;` and escaped characters: \*literal asterisks\*.

```
def function():
    do_a_thing();
end
```

## Links

Inline link: [Test](https:/example.com "Test link")

Reference link: [Reference link test][ref-link]

Autolink: https://example.com/test

[ref-link]: https://www.google.com "Google"

## Images

Inline image: ![Placeholder image](/images/dragon.png "Placeholder")

## Task lists

- [ ] Unchecked item
- [x] Checked item

## Mixed and nested lists

- Unordered item
  - Nested item
    1. Nested ordered item
    2. Nested ordered item
- Unordered item

1. Ordered item
   - Nested unordered item
   - Nested unordered item

## Block quote with list

> Quoted text
> - Quoted list item
> - Quoted list item

## Footnote test

Here is a sentence with a footnote reference.[^note1] Another.[^note2]

[^note1]: Footnote here.
[^note2]: Footnote with *italics* and `inline code`.
