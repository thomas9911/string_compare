#![feature(test)]

extern crate test;

use string_compare::*;
use test::Bencher;

const SMALL_A: &'static str = "hallohallohallohallohallo";
const SMALL_B: &'static str = "hallohallohallohallohallb";

const MEDIUM_A: &'static str =
    "hallohallohallohallohallohallohallohallohallohallohallohallohallohallo";
const MEDIUM_B: &'static str =
    "hallohallohallohallohallohallohallohallohallohallohallohallohallohallb";

const LARGE_A: &'static str = " Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse a viverra ante. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Vestibulum venenatis massa eu orci posuere, sed cursus odio convallis. Cras dignissim tortor laoreet, gravida arcu sit amet, ullamcorper risus. Donec suscipit eros ut nibh vestibulum venenatis vitae sit amet turpis. Etiam a nisi a elit cursus feugiat non nec quam. Sed rhoncus felis id magna aliquet pulvinar. Cras viverra consectetur justo. Sed ornare ligula ac nisl eleifend, sed gravida dolor cursus. Vestibulum in neque et augue faucibus condimentum vel sed justo. Pellentesque faucibus sed libero non vehicula. Aliquam dignissim a diam vel egestas.

In id erat ante. Nullam eget dui erat. Ut vehicula vulputate libero, a sollicitudin nulla cursus quis. Aliquam ac varius justo, id aliquet leo. Donec vel rhoncus est. Interdum et malesuada fames ac ante ipsum primis in faucibus. Morbi id massa in dui malesuada euismod. Mauris convallis felis ut ex pharetra luctus. Vestibulum ultricies justo a volutpat congue. Donec eget felis quis dolor gravida sodales.

Integer non augue nec arcu convallis faucibus imperdiet nec eros. Maecenas accumsan nunc eget tellus venenatis, sit amet pharetra elit consequat. Praesent vestibulum, tellus nec consequat consequat, nulla leo ultricies sapien, vel finibus diam ligula a dolor. Aliquam molestie pulvinar commodo. Etiam pretium mauris a justo vulputate congue. Donec egestas luctus laoreet. Etiam ut velit sit amet quam interdum pellentesque vitae vel augue. Vestibulum ex arcu, ullamcorper id pharetra accumsan, consequat quis enim. Fusce sed mi eu nunc egestas rhoncus quis id mi. Mauris luctus elit nec risus varius hendrerit. Fusce vestibulum ante feugiat erat semper rhoncus.

Integer sed nulla tincidunt, imperdiet lorem et, sollicitudin est. Ut vehicula tincidunt ante, in porta massa pretium eu. Quisque eu nisl libero. Vivamus id commodo diam. Suspendisse laoreet volutpat dui non hendrerit. Praesent id ipsum eget diam mattis dapibus. Aliquam sed leo luctus, consectetur tellus in, pulvinar magna. Sed porta posuere neque accumsan fermentum. Nunc ullamcorper ante ornare mi semper, a scelerisque nulla porta.

Duis feugiat nunc eget est rutrum, in eleifend orci maximus. Aenean consequat, nunc nec interdum dictum, ante urna vestibulum turpis, ut ultricies arcu mi sed sapien. Donec pharetra sem sed orci posuere sodales. Nunc lobortis pharetra quam, vitae ultricies lorem mattis vel. Phasellus efficitur purus quam, nec laoreet libero varius malesuada. Etiam congue convallis fringilla. Proin facilisis neque nec dui eleifend consectetur.

Sed dignissim commodo congue. Vivamus at metus bibendum libero commodo ultrices vitae vel turpis. Donec consequat, tortor id gravida semper, eros ligula commodo erat, quis sollicitudin nulla nulla vitae quam. In vel tristique magna, vel convallis velit. Praesent suscipit dignissim feugiat. Donec dignissim, est eu posuere posuere, mauris risus molestie nulla, in aliquam ipsum mauris porta enim. Fusce ex dolor, pharetra id magna molestie, consectetur viverra ante. Suspendisse potenti. Integer aliquet ex et ipsum ullamcorper, nec porta nulla vulputate. Nulla sit amet eros feugiat, fringilla leo in, tincidunt lorem. Cras pharetra id orci a ornare. Interdum et malesuada fames ac ante ipsum primis in faucibus. Duis eu arcu mauris.

Nam ultrices quis leo at aliquet. Vivamus feugiat fringilla magna. Suspendisse consectetur, lacus auctor luctus laoreet, justo velit consequat dui, ac facilisis mauris ipsum quis augue. Quisque aliquet justo vitae metus euismod cursus. Integer volutpat metus a diam consequat convallis. Aliquam mattis rutrum erat, id mollis tortor suscipit eget. Sed bibendum nulla et iaculis fringilla. Ut ultrices magna eget enim rhoncus, quis scelerisque nulla faucibus. Sed ut convallis sapien. Phasellus augue nibh, sodales vitae rhoncus ut, gravida vel elit. Suspendisse ullamcorper diam ut tellus tempor tempor.

Aliquam erat volutpat. Suspendisse dolor augue, interdum et lacus sed, molestie maximus justo. In hac habitasse platea dictumst. Duis gravida ante vitae neque cursus vulputate. Praesent vel nunc efficitur, accumsan velit at, pulvinar elit. Integer vel tempor nisi. Aliquam dignissim ex vitae quam semper scelerisque. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Cras nec sem non quam pretium imperdiet.

Nam congue in orci non bibendum. Suspendisse potenti. Suspendisse fermentum velit ut metus luctus tincidunt. Pellentesque in convallis nisi. Curabitur suscipit posuere massa sed molestie. Curabitur velit sapien, mollis eget congue a, ornare eget magna. Sed elit est, semper id sem tempor, auctor efficitur odio. Suspendisse auctor, leo dapibus molestie mollis, dui ex ultricies enim, nec dignissim leo urna in sem. Fusce tristique, orci sed faucibus rutrum, quam ipsum maximus tellus, nec euismod erat libero in nisl. Integer vitae porta nisi, eu faucibus urna. Duis eu scelerisque metus. In neque augue, porttitor vel orci quis, euismod venenatis est. Sed tortor mauris, aliquet sed neque ut, ultricies lacinia nisi. Phasellus mi erat, sollicitudin mollis orci ut, cursus auctor tellus. Fusce at tincidunt libero, vel imperdiet purus. Aenean facilisis commodo enim sed lobortis.

Fusce malesuada euismod ullamcorper. Nullam vitae tortor scelerisque, vestibulum magna ac, pulvinar ipsum. Donec eu sapien vel nisi hendrerit congue. Mauris et consequat nisi, id malesuada felis. Integer tristique tortor non dui viverra, nec mollis odio tincidunt. Cras a euismod nulla. Morbi elementum ut dui id ultricies. Morbi ultrices aliquet augue, in molestie ligula. Donec sed purus et nisl vehicula gravida. Mauris eu commodo massa. Nulla dignissim ligula turpis, ut pulvinar diam finibus at. Phasellus finibus dictum ante, quis varius diam rutrum sit amet. Donec pulvinar mollis dolor, non varius dolor faucibus quis. Vivamus pretium bibendum pharetra. Suspendisse bibendum ut justo quis placerat.

Nulla facilisi. Cras molestie vel erat sed suscipit. Donec convallis neque et urna scelerisque, vitae pretium lectus vehicula. Nam tempus felis pulvinar elit molestie, ac condimentum neque facilisis. Mauris commodo mi ut rutrum porttitor. Nulla eget diam eu velit varius tincidunt. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris auctor, erat id finibus vestibulum, ipsum libero auctor velit, ut ultricies nisl leo a augue. Nulla aliquet purus massa, eget vulputate nulla malesuada ac. Quisque augue orci, ultricies a justo et, tristique mollis ante.

Morbi ac nibh et nisl euismod iaculis. Nunc eu odio at tellus fringilla sodales eu et urna. Maecenas eu felis quam. Donec vel est eget urna varius porta. Cras elementum risus ac risus tincidunt imperdiet luctus eu lorem. Suspendisse potenti. Duis mollis lectus et ante pellentesque vestibulum. Donec eu tellus vitae quam dictum venenatis at et ante. Pellentesque elit nisi, suscipit ac fermentum quis, finibus eu ante. Praesent hendrerit hendrerit scelerisque. Pellentesque ac tempus ex. Maecenas ullamcorper congue massa eu dignissim. Nunc in ullamcorper odio. Ut feugiat pellentesque dignissim.

Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Fusce ac mauris ultrices, tempus quam in, volutpat neque. Integer finibus imperdiet mi eget mollis. Nulla cursus, odio quis mattis facilisis, odio tortor tincidunt justo, vitae finibus turpis mi consequat nibh. Etiam elit erat, semper at ligula eget, placerat facilisis mi. Fusce ac vulputate justo. Nullam condimentum, ligula vel interdum dictum, metus eros efficitur sapien, a sodales dui libero et justo.

Praesent et augue mi. Vivamus malesuada, mi nec molestie porta, sapien massa mattis velit, vitae sollicitudin est purus in urna. Aenean luctus urna justo, quis pharetra mauris venenatis id. Donec convallis molestie velit, imperdiet pulvinar dui blandit in. Suspendisse id sodales lectus, sit amet aliquam diam. Nunc volutpat pharetra metus sit amet commodo. Morbi fringilla magna quam. Nulla turpis sapien, varius quis egestas a, sagittis vel dui. Ut congue arcu ante, vel eleifend mauris rutrum eu. Maecenas sagittis tristique ultrices. In molestie sit amet ante in aliquam. Mauris eu suscipit orci, in posuere nulla. In at libero dapibus lectus interdum faucibus.

Integer scelerisque enim augue, quis interdum enim dignissim sed. Donec vitae diam sed sem scelerisque bibendum. Cras ut tellus diam. Sed fringilla augue et augue consectetur ornare. Duis est turpis, ultricies at laoreet in, imperdiet eget mi. Quisque convallis libero eget augue feugiat luctus. Nunc auctor mattis mauris, sed tincidunt neque vehicula non. Curabitur sit amet urna tempus, pulvinar nisl at, semper justo. Proin dapibus metus sed ex posuere maximus. Nunc feugiat quis orci at viverra. Vestibulum efficitur purus at varius finibus. Vestibulum convallis arcu id vulputate dignissim. Praesent augue turpis, placerat in elementum nec, laoreet ut risus. Nullam tortor nisi, commodo nec erat accumsan, lacinia malesuada ligula.

Praesent bibendum in odio nec porttitor. Suspendisse at aliquam velit, in sodales nulla. Curabitur elit lectus, lacinia a dolor eu, ornare semper metus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. In mattis imperdiet imperdiet. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Nulla sit amet condimentum magna. Fusce porta neque neque, a luctus magna sodales vestibulum.

Quisque interdum, eros vel aliquet sagittis, nisi mauris blandit lacus, vel tempor mi nibh eget nisi. Proin mattis luctus aliquet. Maecenas sed pellentesque odio, eget rhoncus tellus. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed finibus, metus vel lobortis vestibulum, magna nibh placerat risus, sed tincidunt neque ex vitae odio. Morbi quis malesuada risus. Mauris eget urna malesuada, facilisis lacus eu, feugiat nunc. Morbi dui nulla, ultrices mattis volutpat sed, interdum scelerisque orci. Aliquam erat volutpat. Maecenas ut molestie quam, non ultrices augue. Cras id maximus massa. Aliquam semper pulvinar mauris ut semper. In elit purus, gravida ut vehicula in, blandit nec erat. Nulla pellentesque metus eu urna consectetur, nec gravida nunc suscipit. Duis at maximus velit. Duis eu elementum urna.

Nullam maximus mollis dignissim. Maecenas orci nisl, pellentesque non purus id, faucibus congue nunc. Pellentesque ultrices magna quis ligula tincidunt fermentum. Sed pretium volutpat interdum. Maecenas vitae pretium dui. Curabitur auctor pretium turpis at congue. In aliquam vel dolor ac congue. Praesent rhoncus urna ac urna laoreet sollicitudin.

Vestibulum aliquet, augue ut rhoncus consectetur, dolor dui accumsan sem, nec vehicula neque eros finibus lorem. Mauris sit amet commodo elit. Sed venenatis nisl quis ipsum suscipit porttitor. Donec congue, ligula venenatis egestas mollis, eros neque ullamcorper massa, in facilisis nunc lorem at ex. Curabitur sit amet mattis ipsum. Donec eleifend nisl a velit sollicitudin, non pellentesque felis mollis. Suspendisse potenti. Etiam iaculis nunc eget nunc tempus posuere. Phasellus ac arcu quis dui tincidunt mattis vel ac quam. Praesent mollis congue felis, aliquam dignissim leo auctor et.

Curabitur eleifend sapien ac magna auctor rhoncus. Curabitur auctor fringilla feugiat. Aliquam consectetur, est in placerat pellentesque, sem urna venenatis elit, ut rutrum felis nibh eu felis. Donec et luctus velit. Vestibulum faucibus, neque et varius feugiat, ante neque tempor tellus, vel mollis enim enim ut ante. Praesent rutrum arcu nec nibh imperdiet, eget elementum mauris interdum. Praesent sed purus mollis, pellentesque massa nec, scelerisque nibh.

Aenean euismod risus sit amet massa cursus, eu ullamcorper purus molestie. Proin auctor felis in quam sollicitudin porta. Donec posuere ligula ut facilisis lacinia. Duis diam turpis, hendrerit sit amet nibh sit amet, imperdiet egestas nunc. Praesent sagittis, justo in iaculis placerat, magna dui accumsan metus, nec volutpat diam ligula a quam. Aenean rhoncus, justo nec ornare pellentesque, quam tellus pulvinar erat, vel lacinia mi arcu vitae ante. Nam non est sapien.

Cras posuere mi ut dictum molestie. Aenean vulputate finibus urna, eu accumsan nunc ornare vitae. Maecenas venenatis quis nibh eu condimentum. Donec nec ante finibus, faucibus magna nec, suscipit velit. Vivamus eros eros, ullamcorper eu consectetur at, malesuada in nunc. Quisque placerat neque eget neque egestas, eget euismod lorem semper. Mauris vel porta metus. In pulvinar sapien urna, in rutrum quam vestibulum non. Maecenas maximus mollis risus, vitae consequat eros condimentum quis. Suspendisse potenti. Etiam vestibulum varius nibh, sed efficitur massa varius luctus. Nullam ac dui enim. Praesent mauris erat, laoreet tincidunt faucibus at, vestibulum tempor lacus. Sed in dui id erat viverra convallis at ut augue. Cras volutpat ante eu tortor fermentum, id posuere metus consequat.

Etiam facilisis porttitor libero. Quisque egestas nulla vitae pulvinar bibendum. In cursus laoreet neque, sit amet semper odio varius quis. Mauris ultricies mi eget quam laoreet, vitae tincidunt quam sagittis. Suspendisse imperdiet tincidunt purus, et mattis erat ultrices a. Vestibulum ac vulputate odio. Nam tortor enim, ultrices ac egestas at, vulputate a sapien. Curabitur egestas est ut leo sagittis finibus.

Mauris maximus metus scelerisque tortor fermentum sodales. Curabitur euismod tristique arcu. Fusce consequat elementum metus quis placerat. Aliquam a sapien metus. Suspendisse eleifend fringilla pulvinar. Vivamus in malesuada erat. Sed sagittis scelerisque lorem, sit amet interdum risus accumsan eget. Duis ut lobortis dui. Donec diam ante, ornare ac porta eu, mollis et ex. Suspendisse vel viverra risus. Proin felis lacus, finibus at nulla nec, aliquam molestie quam. Quisque iaculis elit tellus, in semper magna vulputate sed. Fusce faucibus ante non ligula facilisis sagittis.

Nulla nec commodo dolor, at volutpat erat. Nullam tortor enim, pellentesque at velit eu, rutrum convallis dui. Mauris at est vitae lorem auctor pretium at eget justo. Praesent urna tortor, accumsan et ullamcorper in, bibendum consectetur risus. Curabitur sollicitudin efficitur sollicitudin. Aliquam id suscipit eros. Morbi sem justo, blandit a bibendum vitae, faucibus eget magna. Cras id finibus risus. Nullam vel justo ac odio bibendum elementum. Sed id velit at nisi sagittis sollicitudin. Sed lobortis vestibulum diam eu laoreet. Aenean rhoncus, sem mollis sagittis tincidunt, dolor urna tincidunt orci, ac luctus nunc sapien vel ligula. Praesent lacinia sem at odio consequat congue. Vestibulum nunc urna, cursus eget lorem non, euismod varius augue. ";

const LARGE_B: &'static str = " Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse a viverra ante. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Vestibulum venenatis massa eu orci posuere, sed cursus odio convallis. Cras dignissim tortor laoreet, gravida arcu sit amet, ullamcorper risus. Donec suscipit eros ut nibh vestibulum venenatis vitae sit amet turpis. Etiam a nisi a elit cursus feugiat non nec quam. Sed rhoncus felis id magna aliquet pulvinar. Cras viverra consectetur justo. Sed ornare ligula ac nisl eleifend, sed gravida dolor cursus. Vestibulum in neque et augue faucibus condimentum vel sed justo. Pellentesque faucibus sed libero non vehicula. Aliquam dignissim a diam vel egestas.

In id erat ante. Nullam eget dui erat. Ut vehicula vulputate libero, a sollicitudin nulla cursus quis. Aliquam ac varius justo, id aliquet leo. Donec vel rhoncus est. Interdum et malesuada fames ac ante ipsum primis in faucibus. Morbi id massa in dui malesuada euismod. Mauris convallis felis ut ex pharetra luctus. Vestibulum ultricies justo a volutpat congue. Donec eget felis quis dolor gravida sodales.

Integer non augue nec arcu convallis faucibus imperdiet nec eros. Maecenas accumsan nunc eget tellus venenatis, sit amet pharetra elit consequat. Praesent vestibulum, tellus nec consequat consequat, nulla leo ultricies sapien, vel finibus diam ligula a dolor. Aliquam molestie pulvinar commodo. Etiam pretium mauris a justo vulputate congue. Donec egestas luctus laoreet. Etiam ut velit sit amet quam interdum pellentesque vitae vel augue. Vestibulum ex arcu, ullamcorper id pharetra accumsan, consequat quis enim. Fusce sed mi eu nunc egestas rhoncus quis id mi. Mauris luctus elit nec risus varius hendrerit. Fusce vestibulum ante feugiat erat semper rhoncus.

Integer sed nulla tincidunt, imperdiet lorem et, sollicitudin est. Ut vehicula tincidunt ante, in porta massa pretium eu. Quisque eu nisl libero. Vivamus id commodo diam. Suspendisse laoreet volutpat dui non hendrerit. Praesent id ipsum eget diam mattis dapibus. Aliquam sed leo luctus, consectetur tellus in, pulvinar magna. Sed porta posuere neque accumsan fermentum. Nunc ullamcorper ante ornare mi semper, a scelerisque nulla porta.

Duis feugiat nunc eget est rutrum, in eleifend orci maximus. Aenean consequat, nunc nec interdum dictum, ante urna vestibulum turpis, ut ultricies arcu mi sed sapien. Donec pharetra sem sed orci posuere sodales. Nunc lobortis pharetra quam, vitae ultricies lorem mattis vel. Phasellus efficitur purus quam, nec laoreet libero varius malesuada. Etiam congue convallis fringilla. Proin facilisis neque nec dui eleifend consectetur.

Sed dignissim commodo congue. Vivamus at metus bibendum libero commodo ultrices vitae vel turpis. Donec consequat, tortor id gravida semper, eros ligula commodo erat, quis sollicitudin nulla nulla vitae quam. In vel tristique magna, vel convallis velit. Praesent suscipit dignissim feugiat. Donec dignissim, est eu posuere posuere, mauris risus molestie nulla, in aliquam ipsum mauris porta enim. Fusce ex dolor, pharetra id magna molestie, consectetur viverra ante. Suspendisse potenti. Integer aliquet ex et ipsum ullamcorper, nec porta nulla vulputate. Nulla sit amet eros feugiat, fringilla leo in, tincidunt lorem. Cras pharetra id orci a ornare. Interdum et malesuada fames ac ante ipsum primis in faucibus. Duis eu arcu mauris.

Nam ultrices quis leo at aliquet. Vivamus feugiat fringilla magna. Suspendisse consectetur, lacus auctor luctus laoreet, justo velit consequat dui, ac facilisis mauris ipsum quis augue. Quisque aliquet justo vitae metus euismod cursus. Integer volutpat metus a diam consequat convallis. Aliquam mattis rutrum erat, id mollis tortor suscipit eget. Sed bibendum nulla et iaculis fringilla. Ut ultrices magna eget enim rhoncus, quis scelerisque nulla faucibus. Sed ut convallis sapien. Phasellus augue nibh, sodales vitae rhoncus ut, gravida vel elit. Suspendisse ullamcorper diam ut tellus tempor tempor.

Aliquam erat volutpat. Suspendisse dolor augue, interdum et lacus sed, molestie maximus justo. In hac habitasse platea dictumst. Duis gravida ante vitae neque cursus vulputate. Praesent vel nunc efficitur, accumsan velit at, pulvinar elit. Integer vel tempor nisi. Aliquam dignissim ex vitae quam semper scelerisque. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Cras nec sem non quam pretium imperdiet.

Nam congue in orci non bibendum. Suspendisse potenti. Suspendisse fermentum velit ut metus luctus tincidunt. Pellentesque in convallis nisi. Curabitur suscipit posuere massa sed molestie. Curabitur velit sapien, mollis eget congue a, ornare eget magna. Sed elit est, semper id sem tempor, auctor efficitur odio. Suspendisse auctor, leo dapibus molestie mollis, dui ex ultricies enim, nec dignissim leo urna in sem. Fusce tristique, orci sed faucibus rutrum, quam ipsum maximus tellus, nec euismod erat libero in nisl. Integer vitae porta nisi, eu faucibus urna. Duis eu scelerisque metus. In neque augue, porttitor vel orci quis, euismod venenatis est. Sed tortor mauris, aliquet sed neque ut, ultricies lacinia nisi. Phasellus mi erat, sollicitudin mollis orci ut, cursus auctor tellus. Fusce at tincidunt libero, vel imperdiet purus. Aenean facilisis commodo enim sed lobortis.

Fusce malesuada euismod ullamcorper. Nullam vitae tortor scelerisque, vestibulum magna ac, pulvinar ipsum. Donec eu sapien vel nisi hendrerit congue. Mauris et consequat nisi, id malesuada felis. Integer tristique tortor non dui viverra, nec mollis odio tincidunt. Cras a euismod nulla. Morbi elementum ut dui id ultricies. Morbi ultrices aliquet augue, in molestie ligula. Donec sed purus et nisl vehicula gravida. Mauris eu commodo massa. Nulla dignissim ligula turpis, ut pulvinar diam finibus at. Phasellus finibus dictum ante, quis varius diam rutrum sit amet. Donec pulvinar mollis dolor, non varius dolor faucibus quis. Vivamus pretium bibendum pharetra. Suspendisse bibendum ut justo quis placerat.

Nulla facilisi. Cras molestie vel erat sed suscipit. Donec convallis neque et urna scelerisque, vitae pretium lectus vehicula. Nam tempus felis pulvinar elit molestie, ac condimentum neque facilisis. Mauris commodo mi ut rutrum porttitor. Nulla eget diam eu velit varius tincidunt. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris auctor, erat id finibus vestibulum, ipsum libero auctor velit, ut ultricies nisl leo a augue. Nulla aliquet purus massa, eget vulputate nulla malesuada ac. Quisque augue orci, ultricies a justo et, tristique mollis ante.

Morbi ac nibh et nisl euismod iaculis. Nunc eu odio at tellus fringilla sodales eu et urna. Maecenas eu felis quam. Donec vel est eget urna varius porta. Cras elementum risus ac risus tincidunt imperdiet luctus eu lorem. Suspendisse potenti. Duis mollis lectus et ante pellentesque vestibulum. Donec eu tellus vitae quam dictum venenatis at et ante. Pellentesque elit nisi, suscipit ac fermentum quis, finibus eu ante. Praesent hendrerit hendrerit scelerisque. Pellentesque ac tempus ex. Maecenas ullamcorper congue massa eu dignissim. Nunc in ullamcorper odio. Ut feugiat pellentesque dignissim.

Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Fusce ac mauris ultrices, tempus quam in, volutpat neque. Integer finibus imperdiet mi eget mollis. Nulla cursus, odio quis mattis facilisis, odio tortor tincidunt justo, vitae finibus turpis mi consequat nibh. Etiam elit erat, semper at ligula eget, placerat facilisis mi. Fusce ac vulputate justo. Nullam condimentum, ligula vel interdum dictum, metus eros efficitur sapien, a sodales dui libero et justo.

Praesent et augue mi. Vivamus malesuada, mi nec molestie porta, sapien massa mattis velit, vitae sollicitudin est purus in urna. Aenean luctus urna justo, quis pharetra mauris venenatis id. Donec convallis molestie velit, imperdiet pulvinar dui blandit in. Suspendisse id sodales lectus, sit amet aliquam diam. Nunc volutpat pharetra metus sit amet commodo. Morbi fringilla magna quam. Nulla turpis sapien, varius quis egestas a, sagittis vel dui. Ut congue arcu ante, vel eleifend mauris rutrum eu. Maecenas sagittis tristique ultrices. In molestie sit amet ante in aliquam. Mauris eu suscipit orci, in posuere nulla. In at libero dapibus lectus interdum faucibus.

Integer scelerisque enim augue, quis interdum enim dignissim sed. Donec vitae diam sed sem scelerisque bibendum. Cras ut tellus diam. Sed fringilla augue et augue consectetur ornare. Duis est turpis, ultricies at laoreet in, imperdiet eget mi. Quisque convallis libero eget augue feugiat luctus. Nunc auctor mattis mauris, sed tincidunt neque vehicula non. Curabitur sit amet urna tempus, pulvinar nisl at, semper justo. Proin dapibus metus sed ex posuere maximus. Nunc feugiat quis orci at viverra. Vestibulum efficitur purus at varius finibus. Vestibulum convallis arcu id vulputate dignissim. Praesent augue turpis, placerat in elementum nec, laoreet ut risus. Nullam tortor nisi, commodo nec erat accumsan, lacinia malesuada ligula.

Praesent bibendum in odio nec porttitor. Suspendisse at aliquam velit, in sodales nulla. Curabitur elit lectus, lacinia a dolor eu, ornare semper metus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. In mattis imperdiet imperdiet. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Nulla sit amet condimentum magna. Fusce porta neque neque, a luctus magna sodales vestibulum.

Quisque interdum, eros vel aliquet sagittis, nisi mauris blandit lacus, vel tempor mi nibh eget nisi. Proin mattis luctus aliquet. Maecenas sed pellentesque odio, eget rhoncus tellus. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed finibus, metus vel lobortis vestibulum, magna nibh placerat risus, sed tincidunt neque ex vitae odio. Morbi quis malesuada risus. Mauris eget urna malesuada, facilisis lacus eu, feugiat nunc. Morbi dui nulla, ultrices mattis volutpat sed, interdum scelerisque orci. Aliquam erat volutpat. Maecenas ut molestie quam, non ultrices augue. Cras id maximus massa. Aliquam semper pulvinar mauris ut semper. In elit purus, gravida ut vehicula in, blandit nec erat. Nulla pellentesque metus eu urna consectetur, nec gravida nunc suscipit. Duis at maximus velit. Duis eu elementum urna.

Nullam maximus mollis dignissim. Maecenas orci nisl, pellentesque non purus id, faucibus congue nunc. Pellentesque ultrices magna quis ligula tincidunt fermentum. Sed pretium volutpat interdum. Maecenas vitae pretium dui. Curabitur auctor pretium turpis at congue. In aliquam vel dolor ac congue. Praesent rhoncus urna ac urna laoreet sollicitudin.

Vestibulum aliquet, augue ut rhoncus consectetur, dolor dui accumsan sem, nec vehicula neque eros finibus lorem. Mauris sit amet commodo elit. Sed venenatis nisl quis ipsum suscipit porttitor. Donec congue, ligula venenatis egestas mollis, eros neque ullamcorper massa, in facilisis nunc lorem at ex. Curabitur sit amet mattis ipsum. Donec eleifend nisl a velit sollicitudin, non pellentesque felis mollis. Suspendisse potenti. Etiam iaculis nunc eget nunc tempus posuere. Phasellus ac arcu quis dui tincidunt mattis vel ac quam. Praesent mollis congue felis, aliquam dignissim leo auctor et.

Curabitur eleifend sapien ac magna auctor rhoncus. Curabitur auctor fringilla feugiat. Aliquam consectetur, est in placerat pellentesque, sem urna venenatis elit, ut rutrum felis nibh eu felis. Donec et luctus velit. Vestibulum faucibus, neque et varius feugiat, ante neque tempor tellus, vel mollis enim enim ut ante. Praesent rutrum arcu nec nibh imperdiet, eget elementum mauris interdum. Praesent sed purus mollis, pellentesque massa nec, scelerisque nibh.

Aenean euismod risus sit amet massa cursus, eu ullamcorper purus molestie. Proin auctor felis in quam sollicitudin porta. Donec posuere ligula ut facilisis lacinia. Duis diam turpis, hendrerit sit amet nibh sit amet, imperdiet egestas nunc. Praesent sagittis, justo in iaculis placerat, magna dui accumsan metus, nec volutpat diam ligula a quam. Aenean rhoncus, justo nec ornare pellentesque, quam tellus pulvinar erat, vel lacinia mi arcu vitae ante. Nam non est sapien.

Cras posuere mi ut dictum molestie. Aenean vulputate finibus urna, eu accumsan nunc ornare vitae. Maecenas venenatis quis nibh eu condimentum. Donec nec ante finibus, faucibus magna nec, suscipit velit. Vivamus eros eros, ullamcorper eu consectetur at, malesuada in nunc. Quisque placerat neque eget neque egestas, eget euismod lorem semper. Mauris vel porta metus. In pulvinar sapien urna, in rutrum quam vestibulum non. Maecenas maximus mollis risus, vitae consequat eros condimentum quis. Suspendisse potenti. Etiam vestibulum varius nibh, sed efficitur massa varius luctus. Nullam ac dui enim. Praesent mauris erat, laoreet tincidunt faucibus at, vestibulum tempor lacus. Sed in dui id erat viverra convallis at ut augue. Cras volutpat ante eu tortor fermentum, id posuere metus consequat.

Etiam facilisis porttitor libero. Quisque egestas nulla vitae pulvinar bibendum. In cursus laoreet neque, sit amet semper odio varius quis. Mauris ultricies mi eget quam laoreet, vitae tincidunt quam sagittis. Suspendisse imperdiet tincidunt purus, et mattis erat ultrices a. Vestibulum ac vulputate odio. Nam tortor enim, ultrices ac egestas at, vulputate a sapien. Curabitur egestas est ut leo sagittis finibus.

Mauris maximus metus scelerisque tortor fermentum sodales. Curabitur euismod tristique arcu. Fusce consequat elementum metus quis placerat. Aliquam a sapien metus. Suspendisse eleifend fringilla pulvinar. Vivamus in malesuada erat. Sed sagittis scelerisque lorem, sit amet interdum risus accumsan eget. Duis ut lobortis dui. Donec diam ante, ornare ac porta eu, mollis et ex. Suspendisse vel viverra risus. Proin felis lacus, finibus at nulla nec, aliquam molestie quam. Quisque iaculis elit tellus, in semper magna vulputate sed. Fusce faucibus ante non ligula facilisis sagittis.

Nulla nec commodo dolor, at volutpat erat. Nullam tortor enim, pellentesque at velit eu, rutrum convallis dui. Mauris at est vitae lorem auctor pretium at eget justo. Praesent urna tortor, accumsan et ullamcorper in, bibendum consectetur risus. Curabitur sollicitudin efficitur sollicitudin. Aliquam id suscipit eros. Morbi sem justo, blandit a bibendum vitae, faucibus eget magna. Cras id finibus risus. Nullam vel justo ac odio bibendum elementum. Sed id velit at nisi sagittis sollicitudin. Sed lobortis vestibulum diam eu laoreet. Aenean rhoncus, sem mollis sagittis tincidunt, dolor urna tincidunt orci, ac luctus nunc sapien vel ligula. Praesent lacinia sem at odio consequat congue. Vestibulum nunc urna, cursus eget lorem non, euismod varius augue! ";

#[bench]
fn small_compare_rust(b: &mut Bencher) {
    b.iter(|| {
        let a = test::black_box(SMALL_A);
        let b = test::black_box(SMALL_B);
        compare_rust(a, b)
    });
}

#[bench]
fn small_compare_at_home(b: &mut Bencher) {
    b.iter(|| {
        let a = test::black_box(SMALL_A);
        let b = test::black_box(SMALL_B);
        compare_at_home(a, b)
    });
}

#[bench]
fn small_compare_asm(b: &mut Bencher) {
    b.iter(|| {
        let a = test::black_box(SMALL_A);
        let b = test::black_box(SMALL_B);
        compare_asm(a, b)
    });
}

#[bench]
fn small_compare_simd(b: &mut Bencher) {
    b.iter(|| {
        let a = test::black_box(SMALL_A);
        let b = test::black_box(SMALL_B);
        compare_simd(a, b)
    });
}

#[bench]
fn medium_compare_rust(b: &mut Bencher) {
    b.iter(|| {
        let a = test::black_box(MEDIUM_A);
        let b = test::black_box(MEDIUM_B);
        compare_rust(a, b)
    });
}

#[bench]
fn medium_compare_at_home(b: &mut Bencher) {
    b.iter(|| {
        let a = test::black_box(MEDIUM_A);
        let b = test::black_box(MEDIUM_B);
        compare_at_home(a, b)
    });
}

#[bench]
fn medium_compare_asm(b: &mut Bencher) {
    b.iter(|| {
        let a = test::black_box(MEDIUM_A);
        let b = test::black_box(MEDIUM_B);
        compare_asm(a, b)
    });
}

#[bench]
fn medium_compare_simd(b: &mut Bencher) {
    b.iter(|| {
        let a = test::black_box(MEDIUM_A);
        let b = test::black_box(MEDIUM_B);
        compare_simd(a, b)
    });
}

#[bench]
fn large_compare_rust(b: &mut Bencher) {
    b.iter(|| {
        let a = test::black_box(LARGE_A);
        let b = test::black_box(LARGE_B);
        compare_rust(a, b)
    });
}

#[bench]
fn large_compare_at_home(b: &mut Bencher) {
    b.iter(|| {
        let a = test::black_box(LARGE_A);
        let b = test::black_box(LARGE_B);
        compare_at_home(a, b)
    });
}

#[bench]
fn large_compare_asm(b: &mut Bencher) {
    b.iter(|| {
        let a = test::black_box(LARGE_A);
        let b = test::black_box(LARGE_B);
        compare_asm(a, b)
    });
}

#[bench]
fn large_compare_simd(b: &mut Bencher) {
    b.iter(|| {
        let a = test::black_box(LARGE_A);
        let b = test::black_box(LARGE_B);
        compare_simd(a, b)
    });
}
