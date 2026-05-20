//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2518/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2518<F: Float>(t50948: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43816: F, t43820: F, t50937: F, t50940: F, t50946: F, t50950: F, t50952: F, t50954: F, t50957: F, t50961: F, t50966: F, t50994: F, t51000: F, t51004: F) -> F {
    let t51082 = F::new(8.0) / F::new(9.0) * t50948;
    let t51098 = F::new(6.0) * t50937 + F::new(2.0) / F::new(3.0) * t50940 + F::new(8.0) * t50946 + t51082 + F::new(4.0) / F::new(9.0) * t50950 + F::new(2.0) / F::new(9.0) * t50952 + F::new(4.0) / F::new(3.0) * t50954 - F::new(2.0) / F::new(3.0) * t50957 - F::new(2.0) / F::new(3.0) * t50961 - F::new(4.0) * t50966 + t43820 + F::new(4.0) / F::new(9.0) * t43780 + F::new(8.0) / F::new(9.0) * t43782 + F::new(4.0) / F::new(9.0) * t43784 - F::new(2.0) / F::new(3.0) * t43786 - t43788 / F::new(9.0) - F::new(28.0) / F::new(27.0) * t43816 - F::new(4.0) * t50994 + F::new(6.0) * t51000 + F::new(10.0) / F::new(9.0) * t51004;
    t51098
}
