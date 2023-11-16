#include <zephyr/logging/log.h>
LOG_MODULE_REGISTER(w5500_test, LOG_LEVEL_DBG);

#include <zephyr/kernel.h>
#include <zephyr/net/net_config.h>

int main(void)
{
	LOG_INF("owo");
	while(1) {
		k_sleep(K_MSEC(1000));
	}
}

