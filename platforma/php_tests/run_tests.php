<?php
$output = shell_exec('vendor/bin/phpunit Test.php');
file_put_contents('results_php.txt', $output);
echo "Testet u ekzekutuan me sukses.\n";
