//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 421/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk421<F: Float>(t7625: F, t7640: F, t7647: F, t7652: F, t7656: F, t8143: F, t8156: F, t8784: F, t8786: F, t8788: F, t8790: F, t9453: F, t9465: F, t9477: F, t118: F, t305: F, t8889: F, t8891: F, t8893: F, t8895: F, t8897: F, t8899: F, t8903: F, t8907: F, t8909: F, t8911: F, t8913: F, t8917: F, t9427: F, t9437: F) -> (F, F) {
    let t9484 = -0.21241846568096930143e-2 * t7625 - t8143 - t7640 + t7647 - t7652 + 0.56448240417072397693e-3 * t7656 + 0.5987120850931904282e-1 * t8784 - 0.11974241701863808564e0 * t8786 - t8156 + 0.79656924630363488034e-3 * t8788 - 0.66380770525302906695e-3 * t8790;
    let t9486 = t9453 + t9465 + t9477 + t9484;
    let t9518 = -0.40911992481368012596e-1 * t8889 + 0.81823984962736025192e-1 * t8891 + 0.20455996240684006298e-1 * t8893 + 0.8182398496273602519e-1 * t8895 - 0.13637330827122670865e0 * t8897 - 0.2727466165424534173e-1 * t8899 + 0.20455996240684006298e-1 * t8903 - 0.2727466165424534173e-1 * t8907 - 0.13637330827122670865e-1 * t8909 + 0.59871208509319042821e-1 * t305 * t9437 - 0.39914139006212695214e-1 * t118 * t9427 + 0.54549323308490683461e-1 * t8911 - 0.72732431077987577947e-1 * t8913 - 0.18183107769496894487e-1 * t8917;
    (t9486, t9518)
}
