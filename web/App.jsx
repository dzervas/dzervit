import React, { Component } from 'react';
import DZervit from './components/DZervit.jsx';
import Docs from './components/Docs.jsx';

import Favicon from './img/favicon.ico';
import Logo from './img/logo.png';

class App extends Component{
	constructor(props) {
		super(props);
		this.state = {
			text: "Loading..."
		}
	}

	// component() {
	// 	const logo = new Image();
	// 	logo.src = Logo;
	// 	document.append(logo)
	// }

	componentDidMount() { (2)
		var host = window.location.hostname;
		var anchor = window.location.hash;
		var subdomain = host.split(".");

		if (subdomain.length >= 2) {
			subdomain.pop(); // remove TLD
			subdomain.pop(); // remove domain
		} else {
			subdomain = [];
		}

		var subdomain_text = subdomain.join(", ")
		if (subdomain.length == 0) {
			subdomain_text = "you"
		}

		var anchor_text = "it";
		if (anchor.length > 1) {
			anchor_text = anchor.substring(1); // remove the hash symbol
		}

		var query = new URLSearchParams({
			target: subdomain_text,
			item: anchor_text
		})

		fetch("/api/v1?" + query)
			.then(response => response.json())
			.then(data => this.setState(data))
	}

	render(){
		return (
			<div>
				<header>
					<nav>
						<a href="https://dzerv.it"><img alt="DZervIT" src="./logo.png" height="70" /></a>
						<ul>

							<li><a href="https://whynot.fail/tags/dzervit/" target="_blank">Press</a></li>
							<li><a href="https://github.com/dzervas/dzervit" target="_blank">GitHub</a></li>
						</ul>
					</nav>
					<DZervit text={this.state.text} />
				</header>
				<main>
					<hr />
					<Docs />
				</main>
			</div>
		)
	}
}

export default App;
