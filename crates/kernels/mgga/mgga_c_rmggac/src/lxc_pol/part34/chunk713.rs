//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 713/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk713<F: Float>(t1652: F, t3080: F, t262: F, t8620: F, t41063: F, t739: F, t7577: F, t7778: F, t8946: F, t903: F, t14240: F, t73692: F, t11644: F, t14236: F, t2067: F, t70397: F) -> (F, F, F, F, F, F, F) {
    let t74815 = t3080 * t1652;
    let t74816 = t262 * t74815;
    let t74817 = t8620 * t74816;
    let t74824 = 0.5987120850931904282e-1 * t739 * t7577 * t41063;
    let t74829 = t903 * t7778 * t8946;
    let t74830 = 0.23948483403727617128e0 * t74829;
    let t74831 = t73692 * t14240;
    let t74835 = t14236 * t70397 * t2067 * t11644;
    (t74815, t74816, t74817, t74824, t74830, t74831, t74835)
}
