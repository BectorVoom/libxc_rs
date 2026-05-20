//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2346/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2346<F: Float>(t1553: F, t9709: F, t136: F, t47763: F, t908: F, t47767: F, t13538: F, t699: F, t2826: F, t47684: F, t41831: F, t41833: F, t41863: F, t41865: F, t41870: F, t41872: F, t41874: F, t41876: F, t48085: F, t48087: F, t48090: F, t48092: F, t48097: F, t48098: F, t48101: F) -> (F, F, F, F, F, F) {
    let t48103 = t9709 * t1553;
    let t48112 = t136 * t908 * t47763;
    let t48114 = t136 * t908 * t47767;
    let t48116 = t699 * t13538;
    let t48119 = t136 * t2826 * t47684;
    let t48120 = F::new(2.0) * t48085 - F::new(2.0) * t48087 - t48090 + t48092 / F::new(6.0) - F::new(10.0) / F::new(9.0) * t41831 - F::new(2.0) / F::new(3.0) * t41833 + t48097 - t48098 / F::new(3.0) + t48101 / F::new(6.0) - F::new(40.0) / F::new(81.0) * t48103 - F::new(40.0) / F::new(27.0) * t41863 + F::new(2.0) / F::new(9.0) * t41865 + F::new(5.0) / F::new(9.0) * t41870 + F::new(5.0) / F::new(27.0) * t41872 - t41874 / F::new(9.0) - F::new(4.0) / F::new(81.0) * t41876 - t48112 - t48114 / F::new(3.0) - F::new(4.0) / F::new(27.0) * t48116 - t48119;
    (t48103, t48112, t48114, t48116, t48119, t48120)
}
