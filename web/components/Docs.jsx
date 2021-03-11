import React, { Component } from 'react';

class Docs extends Component {
	render() {
		return (
			<article id="docs">
				<h2>Usage</h2>
				<h3><code>https://{"{"}target{"}"}.dzerv.it/{"{"}item{"}"}</code></h3>
				<h3>Examples:</h3>
				<ul>
					<li><a href="https://you.dzerv.it">you.dzerv.it</a></li>
					<li><a href="https://i.dzerv.it/this">i.dzerv.it/this</a></li>
					<li><a href="https://we.dzerv.it/documentation">we.dzerv.it/documentation</a></li>
				</ul>

				<hr/>

				<h2>API Usage</h2>
				<h3><code>GET /api/v1/?target={"{"}target{"}"}&item={"{"}item{"}"}</code></h3>
				<p>Example response of <code>/api/v1/?target=People&item=happiness</code></p>
				<pre>
					<code>
{"{\n"}
{"    "}"success": true,{"\n"}
{"    "}"text": "People deserve happiness!"{"\n"}
{"}"}
					</code>
				</pre>
			</article>
		)
	}
}

export default Docs;
