//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1190/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1190<F: Float>(t1880: F, t1894: F, t21013: F, t214: F, t1888: F, t232: F, t6646: F, t67358: F, t105531: F, t105543: F, t105547: F, t105551: F, t20853: F, t25255: F, t28351: F, t28409: F, t28413: F, t4166: F, t5575: F, t5585: F, t5617: F, t6657: F, t7535: F, t812: F, t87068: F, t87142: F, t98330: F, t98342: F, t98345: F, t98356: F) -> (F,) {
    let t105561 = t1880 * t214 * t1894 * t21013;
    let t105565 = t1888 * t6646 * t67358 * t232;
    let t105567 = 6.0 * t4166 * t28413 - 3.0 * t4166 * t28409 + 3.0 * t5575 * t7535 - 0.16449340668482264365e-1 * t105531 - 3.0 * t812 * t25255 * t5617 - 6.0 * t4166 * t28351 + 6.0 * t812 * t87142 * t5585 - 0.34543615403812755166e0 * t98330 - 0.24674011002723396548e-1 * t105543 - 0.14804406601634037928e0 * t105547 - 0.9869604401089358619e-1 * t105551 - 0.78134368175290755733e-1 * t87068 - 0.12337005501361698274e-1 * t98342 - t812 * t6657 * t20853 + 0.49348022005446793095e-1 * t98345 + 0.24674011002723396548e-1 * t98356 + 0.82246703342411321825e-2 * t105561 - 0.24674011002723396548e-1 * t105565;
    (t105567,)
}
