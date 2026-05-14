//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 732/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk732<F: Float>(t15707: F, t7376: F, t24851: F, t24574: F, t7365: F, t1235: F, t477: F, t1090: F, t7362: F, t24837: F, t3612: F, t1244: F, t2121: F, t24804: F, t24807: F, t24812: F, t24817: F, t24823: F, t24827: F, t24830: F, t24834: F, t24838: F, t24841: F, t24845: F, t24849: F, t3610: F, t3624: F, t7283: F, t7373: F) -> (F,) {
    let t24852 = t15707 * t7376;
    let t24853 = t24851 * t24852;
    let t24856 = t24574 * t7365;
    let t24858 = t477 * t1235;
    let t24859 = t24858 * t1090;
    let t24860 = t7362 * t24859;
    let t24863 = t24837 * t3612;
    let t24866 = t1244 * t24804 + 0.82246703342411321825e-2 * t7373 * t24807 + 0.16449340668482264365e-1 * t24812 * t24817 - 0.82246703342411321825e-2 * t24812 * t24823 + 0.54831135561607547884e-2 * t24827 + 0.82246703342411321825e-2 * t2121 * t24830 - 0.16449340668482264365e-1 * t7373 * t24834 - t3624 * t24838 + 2.0 * t1244 * t24841 + 0.54831135561607547884e-2 * t24845 - 0.54831135561607547884e-2 * t24849 * t24853 - 0.18277045187202515961e-2 * t24856 - 0.54831135561607547884e-2 * t7283 * t24860 + 2.0 * t3610 * t24863;
    (t24866,)
}
