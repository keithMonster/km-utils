import test from "ava";

import { replaceText } from "../index.js";

test("replaceText", (t) => {
  let original_str = `import { chatPageInitSdk } from '@shared/chat-page';
  import { chatPageInitSdk } from '@shared/chat-page2'`;

  let pattern = "from '@shared/([^']+)'";
  let replacement = "from './_shared/$1'";

  t.is(replaceText(original_str, pattern, replacement), `import { chatPageInitSdk } from './_shared/chat-page';
  import { chatPageInitSdk } from './_shared/chat-page2'`);
});
