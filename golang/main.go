package main

import (
	"fmt"
	"sync"
	"sync/atomic"
	"time"
)

const THREADS = 20
const MESSAGES = 1000000

func main() {
	chennal_counter()
	time.Sleep(time.Second * 2)
	atomic_counter()
}

func atomic_counter() {
	fmt.Println("Atomic")

	start := time.Now()
	var counter uint64

	var wg sync.WaitGroup

	for i := 0; i < THREADS; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for j := 0; j < MESSAGES; j++ {
				atomic.AddUint64(&counter, uint64(1))
			}

		}()
	}

	wg.Wait()

	fmt.Println("Counter", counter)
	fmt.Println("Time taken:", time.Since(start))
}

func channal_counter() {
	fmt.Println("Channal")
	start := time.Now()

	ch := make(chan bool, 5000)
	done := make(chan struct{})

	counter := 0

	go func() {
		for <-ch {
			counter++
		}
		done <- struct{}{}
	}()

	var wg sync.WaitGroup
	for i := 0; i < THREADS; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for j := 0; j < MESSAGES; j++ {
				ch <- true
			}
		}()
	}

	wg.Wait()
	ch <- false
	<-done

	fmt.Println("Counter", counter)
	fmt.Println("Time taken:", time.Since(start))
}
