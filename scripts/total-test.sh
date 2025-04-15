#!/bin/bash

score=0

if ./scripts/test-print.sh ; then
    ((score += 100))
else
    echo "test-print failed"
fi

if ./scripts/test-ramfs_rename.sh ; then
    ((score += 100))
else
    echo "test-ramfs_rename failed"
fi

if ./scripts/test-alt_alloc.sh ; then
    ((score += 100))
else
    echo "test-alt_alloc failed"
fi

if ./scripts/test-support_hashmap.sh ; then
    ((score += 100))
else
    echo "test-support_hashmap failed"
fi

if ./scripts/test-sys_map.sh ; then
    ((score += 100))
else
    echo "test-sys_map failed"
fi

if ./scripts/test-simple_hv.sh ; then
    ((score += 100))
else
    echo "test-simple_hv failed"
fi

echo "$score"
