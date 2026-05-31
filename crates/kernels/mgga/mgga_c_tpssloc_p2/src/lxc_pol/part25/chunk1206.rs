//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1206/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1206<F: Float>(t81849: F, t81852: F, t81855: F, t81857: F, t81859: F, t81861: F, t81863: F, t81866: F, t81869: F, t81874: F, t81877: F, t81880: F, t81883: F, t81887: F, t81889: F, t81891: F, t81893: F, t81895: F, t81899: F, t81903: F) -> F {
    let t84896 = F::cast_from(0.2034786907144675699e0_f64) * t81849;
    let t84897 = F::cast_from(455.0_f64) / F::cast_from(648.0_f64) * t81852;
    let t84916 = -t84896 - t84897 - F::cast_from(0.24223653656484234512e-2_f64) * t81855 - F::cast_from(35.0_f64) / F::cast_from(96.0_f64) * t81857 + F::cast_from(0.84782787797694820791e-2_f64) * t81859 - F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t81861 + t81863 / F::cast_from(64.0_f64) + t81866 / F::cast_from(32.0_f64) - F::cast_from(0.40372756094140390853e-3_f64) * t81869 + F::cast_from(0.20186378047070195427e-3_f64) * t81874 + F::cast_from(0.10093189023535097713e-3_f64) * t81877 + t81880 / F::cast_from(768.0_f64) - F::cast_from(0.31625325607076639502e-2_f64) * t81883 - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t81887 + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t81889 + F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t81891 - t81893 / F::cast_from(256.0_f64) - t81895 / F::cast_from(768.0_f64) + F::cast_from(0.12111826828242117256e-2_f64) * t81899 + F::cast_from(0.60559134141210586279e-3_f64) * t81903;
    t84916
}
