package internal

import (
	"fmt"
	"testing"
	"time"
)

func TestRetryBackoffSaturates(t *testing.T) {
	for _, tc := range []struct {
		attempt uint
		minimum time.Duration
		maximum time.Duration
	}{
		{0, time.Second, 1100 * time.Millisecond},
		{1, 1800 * time.Millisecond, 2200 * time.Millisecond},
		{5, 28800 * time.Millisecond, 35200 * time.Millisecond},
		{6, 54 * time.Second, 60 * time.Second},
		{34, 54 * time.Second, 60 * time.Second},
		{63, 54 * time.Second, 60 * time.Second},
		{^uint(0), 54 * time.Second, 60 * time.Second},
	} {
		t.Run(fmt.Sprint(tc.attempt), func(t *testing.T) {
			defer func() {
				if value := recover(); value != nil {
					t.Errorf("backoff panicked: %v", value)
				}
			}()
			delay, err := NewRetrier().exponentialBackoff(tc.attempt)
			if err != nil || delay < tc.minimum || delay > tc.maximum {
				t.Errorf("delay=%s, err=%v; want [%s, %s]", delay, err, tc.minimum, tc.maximum)
			}
		})
	}
}
