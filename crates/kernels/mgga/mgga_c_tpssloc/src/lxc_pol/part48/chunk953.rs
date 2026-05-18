//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 953/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk953<F: Float>(t112778: F, t112803: F, t112818: F, t112820: F, t112773: F, t112782: F, t112784: F, t112788: F, t112795: F, t112798: F, t112807: F, t112811: F, t112814: F) -> F {
    let t114714 = F::new(0.5383034145885385447e-3) * t112778;
    let t114720 = F::new(7.0) / F::new(576.0) * t112803;
    let t114724 = F::new(0.32298204875312312682e-2) * t112818;
    let t114725 = F::new(7.0) / F::new(144.0) * t112820;
    let t114726 = t112773 / F::new(96.0) + t114714 + F::new(0.67826230238155856632e-1) * t112782 + F::new(0.13565246047631171327e0) * t112784 - F::new(0.96894614625936938046e-2) * t112788 + t112795 / F::new(384.0) - t112798 / F::new(384.0) + t114720 - t112807 / F::new(768.0) - t112811 / F::new(768.0) + F::new(0.32298204875312312682e-2) * t112814 + t114724 + t114725;
    t114726
}
