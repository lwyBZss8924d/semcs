# api.jina.ai/v1/rerank

@docs/tutorials/jina-models/jina-api-v9.txt

## Reranker API

Endpoint: <https://api.jina.ai/v1/rerank>
Purpose: find the most relevant search results
Best for: refining search results, refining RAG (retrieval augmented generation) contextual chunks, etc.
Method: POST
Authorization: HTTPBearer
Headers

- **Authorization**: Bearer $JINA_API_KEY
- **Content-Type**: application/json
- **Accept**: application/json

Request body schema: {"application/json":{"model":{"type":"string","required":true,"description":"Identifier of the model to use.","options":[{"name":"jina-reranker-m0","size":"2.4B"},{"name":"jina-reranker-v2-base-multilingual","size":"278M"},{"name":"jina-colbert-v2","size":"560M"}]},"query":{"type":"string, TextDoc, or image (URL or base64-encoded string)","required":true,"description":"The search query."},"documents":{"type":"If v2 or colbert reranker: array of strings and/or TextDocs. If m0 reranker: object with keys "text" and/or "image", and values of strings, TextDocs, and/or images (URLs or base64-encoded strings)","required":true,"description":"A list of strings, TextDocs, and/or images to rerank. If a document object is provided, all text fields will be preserved in the response. Only jina-reranker-m0 supports images."},"top_n":{"type":"integer","required":false,"description":"The number of most relevant documents or indices to return, defaults to the length of documents."},"return_documents":{"type":"boolean","required":false,"default":true,"description":"If false, returns only the index and relevance score without the document text. If true, returns the index, text, and relevance score."}}}
Example request (v2/colbert): {"model":"jina-reranker-v2-base-multilingual","query":"Search query","documents":["Document to rank 1","Document to rank 2"]}
Example request (m0): {"model":"jina-reranker-m0","query":"small language model data extraction","documents":[{"image":"https://example.com/image1.png"},{"text":"Document to rank 2"}]}
Example response: {"model":"jina-reranker-m0","usage":{"total_tokens":2829},"results":[{"index":0,"relevance_score":0.9587112551898949},{"index":1,"relevance_score":0.9337408271911014}]}

## API Model Task Parameters

- "model" : ("jina-reranker-v3", "jina-reranker-m0")
- "query" : "strings"
- "documents" : ["strings"] (a list of candidate documents strings to rank)
- "top_n" : integer (Number of returned documents: The number of most relevant documents to return for the query.)
- "return_documents" : boolean

## Example

### Example Request

```shell
curl https://api.jina.ai/v1/rerank \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer jina_b97f3746e9774f88880f72e61172de3dpDc39t2-u1BBUBo7iE6grS3xrATa" \
  -d @- <<EOFEOF
  {
    "model": "jina-reranker-v3",
    "query": "Organic skincare products for sensitive skin",
    "top_n": 3,
    "documents": [
        "Organic skincare for sensitive skin with aloe vera and chamomile: Imagine the soothing embrace of nature with our organic skincare range, crafted specifically for sensitive skin. Infused with the calming properties of aloe vera and chamomile, each product provides gentle nourishment and protection. Say goodbye to irritation and hello to a glowing, healthy complexion.",
        "New makeup trends focus on bold colors and innovative techniques: Step into the world of cutting-edge beauty with this seasons makeup trends. Bold, vibrant colors and groundbreaking techniques are redefining the art of makeup. From neon eyeliners to holographic highlighters, unleash your creativity and make a statement with every look.",
        "Bio-Hautpflege für empfindliche Haut mit Aloe Vera und Kamille: Erleben Sie die wohltuende Wirkung unserer Bio-Hautpflege, speziell für empfindliche Haut entwickelt. Mit den beruhigenden Eigenschaften von Aloe Vera und Kamille pflegen und schützen unsere Produkte Ihre Haut auf natürliche Weise. Verabschieden Sie sich von Hautirritationen und genießen Sie einen strahlenden Teint.",
        "Neue Make-up-Trends setzen auf kräftige Farben und innovative Techniken: Tauchen Sie ein in die Welt der modernen Schönheit mit den neuesten Make-up-Trends. Kräftige, lebendige Farben und innovative Techniken setzen neue Maßstäbe. Von auffälligen Eyelinern bis hin zu holografischen Highlightern – lassen Sie Ihrer Kreativität freien Lauf und setzen Sie jedes Mal ein Statement.",
        "Cuidado de la piel orgánico para piel sensible con aloe vera y manzanilla: Descubre el poder de la naturaleza con nuestra línea de cuidado de la piel orgánico, diseñada especialmente para pieles sensibles. Enriquecidos con aloe vera y manzanilla, estos productos ofrecen una hidratación y protección suave. Despídete de las irritaciones y saluda a una piel radiante y saludable.",
        "Las nuevas tendencias de maquillaje se centran en colores vivos y técnicas innovadoras: Entra en el fascinante mundo del maquillaje con las tendencias más actuales. Colores vivos y técnicas innovadoras están revolucionando el arte del maquillaje. Desde delineadores neón hasta iluminadores holográficos, desata tu creatividad y destaca en cada look.",
        "针对敏感肌专门设计的天然有机护肤产品：体验由芦荟和洋甘菊提取物带来的自然呵护。我们的护肤产品特别为敏感肌设计，温和滋润，保护您的肌肤不受刺激。让您的肌肤告别不适，迎来健康光彩。",
        "新的化妆趋势注重鲜艳的颜色和创新的技巧：进入化妆艺术的新纪元，本季的化妆趋势以大胆的颜色和创新的技巧为主。无论是霓虹眼线还是全息高光，每一款妆容都能让您脱颖而出，展现独特魅力。",
        "敏感肌のために特別に設計された天然有機スキンケア製品: アロエベラとカモミールのやさしい力で、自然の抱擁を感じてください。敏感肌用に特別に設計された私たちのスキンケア製品は、肌に優しく栄養を与え、保護します。肌トラブルにさようなら、輝く健康な肌にこんにちは。",
        "新しいメイクのトレンドは鮮やかな色と革新的な技術に焦点を当てています: 今シーズンのメイクアップトレンドは、大胆な色彩と革新的な技術に注目しています。ネオンアイライナーからホログラフィックハイライターまで、クリエイティビティを解き放ち、毎回ユニークなルックを演出しましょう。"
    ],
    "return_documents": false
  }
EOFEOF
```