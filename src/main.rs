
/// filename backup
// #if($challenge.slug)$!velocity.camelCaseName($challenge.slug)#elseif($challenge.name)    $!velocity.camelCaseName($challenge.name)#elseif($challenge.title)    $!velocity.camelCaseName($challenge.title)#else    "Unknown"#end

/// plugin backup template

/*
struct Solution;

#set($rustCode = ${challenge.code})
#set($pubFnIndex = $rustCode.indexOf("pub fn "))
#set($afterPubFn = $rustCode.substring($pubFnIndex + 7))
#set($parenIndex = $afterPubFn.indexOf("("))
#set($methodName = $afterPubFn.substring(0, $parenIndex).trim())

fn main() {
    let mut cases = vec![];

    cases.push(Solution::$methodName());
    cases.push(Solution::$methodName());
    cases.push(Solution::$methodName());

    println!("{:?}", cases)
}

${challenge.code}
 */

fn main() {}