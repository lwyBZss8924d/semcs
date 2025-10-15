# api.jina.ai/v1/embeddings

@docs/tutorials/jina-models/jina-api-v9.txt

## Embeddings API

Endpoint: <https://api.jina.ai/v1/embeddings>
Purpose: Convert text/images to fixed-length vectors
Best for: semantic search, similarity matching, clustering, etc.
Method: POST
Authorization: HTTPBearer
Headers

- **Authorization**: Bearer $JINA_API_KEY
- **Content-Type**: application/json
- **Accept**: application/json

Jina Code Embeddings Models:
`jina-code-embeddings-0.5b` is a 494 million parameter code embedding model designed for retrieving code from natural language queries, technical Q&A, and identifying similar code across languages.
Built on Qwen2.5-Coder-0.5B backbone, it generates embeddings via last-token pooling and addresses the fundamental limitation of traditional code embedding models that rely on scarce aligned data like comments and docstrings.
`jina-code-embeddings-1.5b` is a 1.54 billion parameter model representing a significant advancement in code retrieval capabilities.
Built on Qwen2.5-Coder-1.5B backbone with last-token pooling, it moves beyond traditional training on limited aligned data to leverage vast unaligned code and documentation corpora.

Both code embeddings models implement comprehensive task-specific instructions across five categories: NL2Code, TechQA, Code2Code, Code2NL, and Code2Completion, each with distinct prefixes for queries and documents.
Supports Matryoshka representation learning for flexible embedding truncation. Despite larger size, maintains practical deployment characteristics while achieving benchmark performance competitive with substantially larger alternatives.

Request body schema for jina-code-embeddings-0.5b or jina-code-embeddings-1.5b:
{
 "application/json": {
   "model":{"type":"string","required":true,"description":"Identifier of the model to use."},
   "input":{"type":"array","required":true,"description":"Array of input strings or objects to be embedded."},
   "embedding_type": {
    "type":"string or array of strings",
    "required":false,"default":"float",
    "description":"The format of the returned embeddings.",
    "options":["float","base64","binary","ubinary"]
   },
   "task":{
    "type":"string",
    "required":false,
    "description":"Specifies the intended downstream application to optimize embedding output.",
    "options": [
     "nl2code.query","nl2code.passage",
     "code2code.query","code2code.passage",
     "code2nl.query","code2nl.passage",
     "code2completion.query","code2completion.passage",
     "qa.query","qa.passage",
    ]
   },
   "dimensions":{"type":"integer","required":false,"description":"Truncates output embeddings to the specified size if set."},
   "truncate":{"type":"boolean","required":false,"default":false,"description":"If true, the model will automatically drop the tail that extends beyond the maximum context length allowed by the model instead of throwing an error."}
 }
}
Example request1: {"model":"jina-code-embeddings-0.5b","input":["Calculates the square of a number. Parameters: number (int or float) - The number to square. Returns: int or float - The square of the number."], task: "nl2code.query"}
Example request2: {"model":"jina-code-embeddings-1.5b","input":["import * as ElementPlusIconsVue from '@element-plus/icons-vue'\nconst app = createApp(App)\nfor (const [key, component] of Object.entries(ElementPlusIconsVue)) {\napp.component(key, component)\n}"], task: "nl2code.passage"}
Example response: {"200":{"data":[{"embedding":"..."}],"usage":{"total_tokens":15}},"422":{"error":{"message":"Invalid input or parameters"}}}

Jina Embeddings Models:
Request body schema for jina-embeddings-v4: {"application/json":{"model":{"type":"string","required":true,"description":"Identifier of the model to use. `jina-embeddings-v4` is a multimodal and multilingual model with a model size of 3.8B and output dimensions of 2048."},"input":{"type":"array","required":true,"description":"Array of input strings or objects to be embedded."},"embedding_type":{"type":"string or array of strings","required":false,"default":"float","description":"The format of the returned embeddings.","options":["float","base64","binary","ubinary"]},"task":{"type":"string","required":false,"description":"Specifies the intended downstream application to optimize embedding output.","options":["retrieval.query","retrieval.passage","text-matching","code.query","code.passage"]},"dimensions":{"type":"integer","required":false,"description":"Truncates output embeddings to the specified size if set."},"late_chunking":{"type":"boolean","required":false,"default":false,"description":"If true, concatenates all sentences in input and treats as a single input for late chunking."},"truncate":{"type":"boolean","required":false,"default":false,"description":"If true, the model will automatically drop the tail that extends beyond the maximum context length allowed by the model instead of throwing an error."},"return_multivector":{"type":"boolean","required":false,"default":false,"description":"If true, the model will return NxD multi-vector embeddings for every document, where N is the number of tokens in the document. Useful for late interaction style retrieval."}}}
Example request1: {"model":"jina-embeddings-v4","input":["Hello, world!"]}
Example request2: {"model":"jina-embeddings-v4","input":[{"text": ""Hello, world!"},{"image": "https://i.ibb.co/r5w8hG8/beach2.jpg"},{"image": "iVBORw0KGgoAAAANSUhEUgAAABwAAAA4CAIAAABhUg/jAAAAMklEQVR4nO3MQREAMAgAoLkoFreTiSzhy4MARGe9bX99lEqlUqlUKpVKpVKpVCqVHksHaBwCA2cPf0cAAAAASUVORK5CYII="}]}
Example request3: {"model":"jina-embeddings-v4","input":{"pdf":"<https://jina.ai/Jina%20AI%20GmbH_Letter%20of%20Attestation%20SOC%202%20Type%202.pdf"}}>
Example response: {"200":{"data":[{"embedding":"..."}],"usage":{"total_tokens":15}},"422":{"error":{"message":"Invalid input or parameters"}}}

Request body schema for jina-embeddings-v3 or jina-clip-v2: {"application/json":{"model":{"type":"string","required":true,"description":"Identifier of the model to use.","options":[{"name":"jina-clip-v2","size":"885M","dimensions":1024},{"name":"jina-embeddings-v3","size":"570M","dimensions":1024}]},"input":{"type":"array","required":true,"description":"Array of input strings or objects to be embedded."},"embedding_type":{"type":"string or array of strings","required":false,"default":"float","description":"The format of the returned embeddings.","options":["float","base64","binary","ubinary"]},"task":{"type":"string","required":false,"description":"Specifies the intended downstream application to optimize embedding output.","options":["retrieval.query","retrieval.passage","text-matching","classification","separation"]},"dimensions":{"type":"integer","required":false,"description":"Truncates output embeddings to the specified size if set."},"normalized":{"type":"boolean","required":false,"default":false,"description":"If true, embeddings are normalized to unit L2 norm."},"late_chunking":{"type":"boolean","required":false,"default":false,"description":"If true, concatenates all sentences in input and treats as a single input for late chunking."},"truncate":{"type":"boolean","required":false,"default":false,"description":"If true, the model will automatically drop the tail that extends beyond the maximum context length allowed by the model instead of throwing an error."}}}
Example request: {"model":"jina-embeddings-v3","input":["Hello, world!"]}
Example response: {"200":{"data":[{"embedding":"..."}],"usage":{"total_tokens":15}},"422":{"error":{"message":"Invalid input or parameters"}}}

## API Model Task Parameters

### Downstream task

Our embeddings are general-purpose and excel at popular tasks. Once a task is set, they deliver highly optimized embeddings tailored to the task.

nl2code.query
Find the most relevant code snippet given the following query.

nl2code.passage
Candidate code snippet.

code2code.query
Find an equivalent code snippet given the following code snippet.

code2code.passage
Candidate code snippet.

code2nl.query
Find the most relevant comment given the following code snippet.

code2nl.passage
Candidate comment.

code2completion.query
Find the most relevant completion given the following start of code snippet.

code2completion.passage
Candidate completion

qa.query
Find the most relevant answer given the following question.

qa.passage
Candidate answer.

### truncate

Truncate at maximum length

When enabled, the model will automatically drop the tail that extends beyond the maximum context length of 8192 tokens allowed by the model instead of throwing an error.

`"truncate": true,` or `"truncate": false,`

### Output data type

Instead of float, you can set it to binary for faster vector retrieval, or as base64 encoding for faster transmission.

Default (as float)
The embeddings are returned as a list of floating-point numbers. Most common and easy to use.

Binary (packed as int8)
The embeddings are packed as int8. Much more efficient for storage, search and transmission.

Binary (packed as uint8)
The embeddings are packed as uint8. Much more efficient for storage, search and transmission.

Base64 (as string)
The embeddings are returned as a base64-encoded string. More efficient for transmission.

## Example

### Example Request

```shell
curl <https://api.jina.ai/v1/embeddings> \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer jina_b97f3746e9774f88880f72e61172de3dpDc39t2-u1BBUBo7iE6grS3xrATa" \
  -d @- <<EOFEOF
  {
    "model": "jina-code-embeddings-1.5b",
    "task": "nl2code.query",
    "truncate": true,
    "input": [
        "Calculates the square of a number. Parameters: number (int or float) - The number to square. Returns: int or float - The square of the number.",
        "This function calculates the square of a number you give it.",
        "def square(number): return number ** 2",
        "print(square(5))",
        "Output: 25",
        "Each text can be up to 8192 tokens long"
    ]
  }
EOFEOF
```

### Example Response

```json
{
  "model": "jina-code-embeddings-1.5b",
  "object": "list",
  "usage": {
    "total_tokens": 144,
    "prompt_tokens": 144
  },
  "data": [
    {
      "object": "embedding",
      "index": 0,
      "embedding": [
        0.01773242,
        -0.01576117,
        -0.00815119,
        0.04618692,
        -0.02336077,
        ...
      ]
    },
    {
      "object": "embedding",
      "index": 1,
      "embedding": [
        0.02236189,
        -0.02978932,
        0.00141916,
        0.04396442,
        -0.00914115,
        ...
      ]
    },
    {
      "object": "embedding",
      "index": 2,
      "embedding": [
        0.022509,
        -0.00562286,
        0.01468558,
        0.03888507,
        -0.01671905,
        ...
      ]
    },
    {
      "object": "embedding",
      "index": 3,
      "embedding": [
        0.01914663,
        -0.01116125,
        0.0165137,
        0.05794434,
        -0.00340263,
        ...
      ]
    },
    {
      "object": "embedding",
      "index": 4,
      "embedding": [
        0.02169601,
        0.01483658,
        0.02686701,
        0.04176365,
        0.04068765,
        ...
      ]
    },
    {
      "object": "embedding",
      "index": 5,
      "embedding": [
        -0.0013359,
        -0.01883088,
        0.0255413,
        0.0048121,
        0.02548533,
        ...
      ]
    }
  ]
}
```
