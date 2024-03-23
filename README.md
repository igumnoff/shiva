# Shiva

![shiva](logo.png)

**Shiva library: Implementation in Rust of a parser and generator for documents of any type**

## Supported document types

| Document type | Parse | Generate |
|---------------|-------|----------|
| Plain text    | +     | +        |
| Markdown      | +     | +        |
| HTML          | +     | +        |
| PDF           | +     | -        |
| JSON          | -     | -        |
| XML           | -     | -        |
| DOC           | -     | -        |
| XLS           | -     | -        |


## Parse document features

| Document type | Header | Paragraph | List | Table | Image | Hyperlink |
|---------------|--------|-----------|------|-------|-------|-----------|
| Plain text    | -      | +         | -    | -     | -     | -         |
| Markdown      | +      | +         | +    | +     | +     | +         |
| HTML          | +      | +         | +    | +     | +     | +         |
| PDF           | -      | +         | +    | -     | -     | -         |

## Generate document features

| Document type | Header | Paragraph  | List | Table | Image | Hyperlink |
|---------------|--------|------------|------|-------|-------|-----------|
| Plain text    | +      | +          | +    | +     | -     | +         |
| Markdown      | +      | +          | +    | +     | +     | +         |
| HTML          | +      | +          | +    | +     | +     | +         |
| PDF           | -      | -          | -    | -     | -     | -         |

## Usage Shiva library

TODO

## Shiva CLI

TODO