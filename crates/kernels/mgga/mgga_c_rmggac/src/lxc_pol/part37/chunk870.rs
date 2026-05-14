//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 870/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk870<F: Float>(t14562: F, t14566: F, t14570: F, t14571: F, t14574: F, t14577: F, t14579: F, t14583: F, t14587: F, t15460: F, t15861: F, t15889: F, t70680: F, t70681: F, t73645: F, t13998: F, t14591: F, t14592: F, t14593: F, t14594: F, t14595: F, t14596: F, t14598: F, t14607: F, t14955: F, t14956: F, t14957: F, t14958: F, t14959: F, t14960: F, t14961: F) -> (F, F) {
    let t79950 = -t15460 + t15861 + t73645 + t70680 - t70681 + t14562 + t14566 - t14570 + t14571 - t14574 + t14577 + t15889 + t14579 - t14583 + t14587;
    let t79951 = t14591 + t14592 + t14593 - t14594 + t14595 + t13998 - t14955 + t14596 + t14956 + t14598 + t14957 - t14958 + t14959 + t14960 + t14961 + t14607;
    (t79950, t79951)
}
