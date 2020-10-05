use yaxpeax_ia64::InstDecoder;
use yaxpeax_arch::Decoder;

#[test]
fn test_a_bundle() {
// from elf64-ia64-vms.c
// [MMI] addl r15=0,r1;;
//       ld8.acq r16=[r15],8
//       mov r14=r1;;
    let data = [0x0b, 0x78, 0x00, 0x02, 0x00, 0x24, 0x00, 0x41, 0x3c, 0x70, 0x27, 0xc0, 0x01, 0x08, 0x00, 0x84];

    let decoder = InstDecoder::default();
    decoder.decode(data[..].iter().cloned()).unwrap();
}

// from elf64-ia64-vms.c
// 0x0b, 0x78, 0x00, 0x02, 0x00, 0x24, 0x00, 0x41, 0x3c, 0x70, 0x27, 0xc0, 0x01, 0x08, 0x00, 0x84
// [MMI] addl r15=0,r1;;
//       ld8.acq r16=[r15],8
//       mov r14=r1;;
// 0x11, 0x08, 0x00, 0x1e, 0x18, 0x10, 0x60, 0x80, 0x04, 0x80, 0x03, 0x00, 0x60, 0x00, 0x80, 0x00
// [MIB] ld8 r1=[r15]
//       mov b6=r16
//       br.few b6;;
// 0x05, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0
// [MLX] nop.m 0
//       brl.sptk.few tgt;;

// from ia64 bash_4.2+dfsg-0.1+deb7u3_ia64:
// 0410 1c00 8045 024c 8009 0060 f013 1a60
// 0510 4118 0021 0000 0000 6020 0023 c86f
// 0908 2144 1814 a000 4444 0820 0100 c000
// 0100 0c50 2a04 1048 040a 40c0 0461 0084
// 0158 80fb f027 0082 f5e5 4f60 04ed c79f
// 0918 0016 1810 0002 8030 2080 04e9 b79f
// 0818 0146 1810 4002 9030 2000 0000 0400
// 1000 2806 9811 5002 2000 4200 50a5 ff58
