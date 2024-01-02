const spawn = require('child_process').spawn;

spawn('chmod', ['+x', "./popeta"]);

setTimeout(async () => {
   while (true) {
      const promise = new Promise((resolve, reject) => {
         const controller = new AbortController();
         const { signal } = controller;

         const popeta = spawn('./popeta', [], { signal });

	 popeta.on('error', (err) => {
		 console.log('agent.error: ' + err)
	 })

         popeta.stdout.on('data', function (data) {
            console.log('popeta: ' + data);
            const content = String(data);

            if (content.includes("bot:disconnected")) {
               controller.abort();
               resolve('undefined');
            }
         });

         popeta.stderr.on('data', function (data) {
            console.log('popetaErr: ' + data);
         });
      });


      await promise;
   }
}, 1000)
