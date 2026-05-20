//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2605/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2605<F: Float>(t1174: F, t135: F, t22011: F, t18375: F, t5019: F, t1216: F, t18946: F, t19033: F, t19056: F, t19083: F, t22208: F, t3490: F, t3506: F, t44836: F, t4582: F, t4950: F, t4954: F, t4989: F, t5030: F, t65884: F, t65952: F, t65992: F, t65994: F, t65996: F, t65998: F, t72445: F) -> F {
    let t72669 = t1174 * t135 * t22011;
    let t72673 = t5019 * t18375;
    let t72683 = -t44836 * t4582 * t72445 * t1216 / F::new(3072.0) - t65952 / F::new(576.0) + t19083 * t5030 / F::new(144.0) - F::new(5.0) / F::new(5184.0) * t3490 * t22208 + t3506 * t4582 * t19056 * t18946 / F::new(512.0) - F::new(7.0) / F::new(1944.0) * t72669 + F::new(95.0) / F::new(2592.0) * t19033 * t4989 - t72673 / F::new(288.0) - t65992 / F::new(144.0) - t65994 / F::new(144.0) + t65996 / F::new(768.0) + t65998 / F::new(768.0) + t65884 * t4950 / F::new(144.0) + t65884 * t4954 / F::new(144.0);
    t72683
}
