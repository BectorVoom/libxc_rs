//! GGA_K_PG lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_pg.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use crate::math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use crate::math::piecewise::{piecewise3, piecewise5};
use crate::math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_pg_lxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    param_pg_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = M_CBRT6;
        let t25 = M_PI * M_PI;
        let t26 = pow_1_3(t25);
        let t27 = t26 * t26;
        let t28 = 1.0 / t27;
        let t29 = t24 * t28;
        let t30 = M_CBRT2;
        let t31 = t30 * t30;
        let t32 = sigma[ip] * t31;
        let t33 = rho[ip] * rho[ip];
        let t35 = 1.0 / t22 / t33;
        let t36 = t32 * t35;
        let t40 = param_pg_mu * t24 * t28;
        let t43 = f64::exp(-t40 * t36 / 24.0);
        let t44 = 5.0 / 72.0 * t29 * t36 + t43;
        let t48 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t44);
        let tzk0 = 2.0 * t48;
        zk[ip] += tzk0;
        let t50 = t20 / t21;
        let t54 = t33 * rho[ip];
        let t56 = 1.0 / t22 / t54;
        let t64 = -5.0 / 27.0 * t29 * t32 * t56 + t40 * t32 * t56 * t43 / 9.0;
        let t69 = piecewise3(t2, 0.0, t7 * t50 * t44 / 10.0 + 3.0 / 20.0 * t7 * t23 * t64);
        let tvrho0 = 2.0 * rho[ip] * t69 + 2.0 * t48;
        vrho[ip] += tvrho0;
        let t72 = t31 * t35;
        let t78 = 5.0 / 72.0 * t29 * t72 - t40 * t72 * t43 / 24.0;
        let t82 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t78);
        let tvsigma0 = 2.0 * rho[ip] * t82;
        vsigma[ip] += tvsigma0;
        let t87 = t20 / t21 / rho[ip];
        let t94 = t33 * t33;
        let t96 = 1.0 / t22 / t94;
        let t104 = param_pg_mu * param_pg_mu;
        let t105 = t24 * t24;
        let t106 = t104 * t105;
        let t108 = 1.0 / t26 / t25;
        let t109 = t106 * t108;
        let t110 = sigma[ip] * sigma[ip];
        let t111 = t110 * t30;
        let t114 = 1.0 / t21 / t94 / t54;
        let t119 = 55.0 / 81.0 * t29 * t32 * t96 - 11.0 / 27.0 * t40 * t32 * t96 * t43 + 2.0 / 81.0 * t109 * t111 * t114 * t43;
        let t124 = piecewise3(t2, 0.0, -t7 * t87 * t44 / 30.0 + t7 * t50 * t64 / 5.0 + 3.0 / 20.0 * t7 * t23 * t119);
        let tv2rho20 = 2.0 * rho[ip] * t124 + 4.0 * t69;
        v2rho2[ip] += tv2rho20;
        let t130 = t31 * t56;
        let t136 = t94 * t33;
        let t140 = sigma[ip] * t43;
        let t144 = -5.0 / 27.0 * t29 * t130 + t40 * t130 * t43 / 9.0 - t109 * t30 / t21 / t136 * t140 / 108.0;
        let t149 = piecewise3(t2, 0.0, t7 * t50 * t78 / 10.0 + 3.0 / 20.0 * t7 * t23 * t144);
        let tv2rhosigma0 = 2.0 * rho[ip] * t149 + 2.0 * t82;
        v2rhosigma[ip] += tv2rhosigma0;
        let t156 = t106 * t108 * t30 * t43;
        let t159 = piecewise3(t2, 0.0, t7 * t20 * t96 * t156 / 1920.0);
        let tv2sigma20 = 2.0 * rho[ip] * t159;
        v2sigma2[ip] += tv2sigma20;
        let t164 = t20 / t21 / t33;
        let t174 = t94 * rho[ip];
        let t176 = 1.0 / t22 / t174;
        let t184 = t94 * t94;
        let t186 = 1.0 / t21 / t184;
        let t191 = t104 * param_pg_mu;
        let t192 = t25 * t25;
        let t193 = 1.0 / t192;
        let t194 = t191 * t193;
        let t195 = t110 * sigma[ip];
        let t197 = 1.0 / t184 / t54;
        let t202 = -770.0 / 243.0 * t29 * t32 * t176 + 154.0 / 81.0 * t40 * t32 * t176 * t43 - 22.0 / 81.0 * t109 * t111 * t186 * t43 + 8.0 / 243.0 * t194 * t195 * t197 * t43;
        let t207 = piecewise3(t2, 0.0, 2.0 / 45.0 * t7 * t164 * t44 - t7 * t87 * t64 / 10.0 + 3.0 / 10.0 * t7 * t50 * t119 + 3.0 / 20.0 * t7 * t23 * t202);
        let tv3rho30 = 2.0 * rho[ip] * t207 + 6.0 * t124;
        v3rho3[ip] += tv3rho30;
        let t217 = t31 * t96;
        let t228 = 1.0 / t184 / t33;
        let t233 = 55.0 / 81.0 * t29 * t217 - 11.0 / 27.0 * t40 * t217 * t43 + t109 * t30 * t114 * t140 / 12.0 - t194 * t228 * t110 * t43 / 81.0;
        let t238 = piecewise3(t2, 0.0, -t7 * t87 * t78 / 30.0 + t7 * t50 * t144 / 5.0 + 3.0 / 20.0 * t7 * t23 * t233);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t238 + 4.0 * t149;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t245 = t5 * t5;
        let t248 = t4 / t245 / t25;
        let t249 = t248 * t20;
        let t250 = t186 * t191;
        let t255 = piecewise3(t2, 0.0, -7.0 / 2880.0 * t7 * t20 * t176 * t156 + t249 * t250 * t140 / 1440.0);
        let tv3rhosigma20 = 2.0 * rho[ip] * t255 + 2.0 * t159;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t262 = piecewise3(t2, 0.0, -t249 * t114 * t191 * t43 / 3840.0);
        let tv3sigma30 = 2.0 * rho[ip] * t262;
        v3sigma3[ip] += tv3sigma30;
        let t281 = 1.0 / t22 / t136;
        let t291 = 1.0 / t21 / t184 / rho[ip];
        let t297 = 1.0 / t184 / t94;
        let t302 = t104 * t104;
        let t303 = t302 * t193;
        let t304 = t110 * t110;
        let t311 = t29 * t31 * t43;
        let t319 = piecewise3(t2, 0.0, -14.0 / 135.0 * t7 * t20 / t21 / t54 * t44 + 8.0 / 45.0 * t7 * t164 * t64 - t7 * t87 * t119 / 5.0 + 2.0 / 5.0 * t7 * t50 * t202 + 3.0 / 20.0 * t7 * t23 * (13090.0 / 729.0 * t29 * t32 * t281 - 2618.0 / 243.0 * t40 * t32 * t281 * t43 + 1958.0 / 729.0 * t109 * t111 * t291 * t43 - 176.0 / 243.0 * t194 * t195 * t297 * t43 + 8.0 / 2187.0 * t303 * t304 / t22 / t184 / t136 * t311));
        let tv4rho40 = 2.0 * rho[ip] * t319 + 8.0 * t207;
        v4rho4[ip] += tv4rho40;
        let t332 = t31 * t176;
        let t358 = piecewise3(t2, 0.0, 2.0 / 45.0 * t7 * t164 * t78 - t7 * t87 * t144 / 10.0 + 3.0 / 10.0 * t7 * t50 * t233 + 3.0 / 20.0 * t7 * t23 * (-770.0 / 243.0 * t29 * t332 + 154.0 / 81.0 * t40 * t332 * t43 - 341.0 / 486.0 * t109 * t30 * t186 * t140 + 19.0 / 81.0 * t194 * t197 * t110 * t43 - t303 / t22 / t184 / t174 * t195 * t311 / 729.0));
        let tv4rho3sigma0 = 2.0 * rho[ip] * t358 + 6.0 * t238;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t375 = t28 * t31 * t43;
        let t380 = piecewise3(t2, 0.0, 119.0 / 8640.0 * t7 * t20 * t281 * t156 - 13.0 / 1440.0 * t249 * t291 * t191 * t140 + t248 * t20 * t297 * t302 * t110 * t24 * t375 / 12960.0);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t380 + 4.0 * t255;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t394 = piecewise3(t2, 0.0, 11.0 / 5760.0 * t249 * t250 * t43 - t248 * t20 * t197 * t302 * t29 * t32 * t43 / 34560.0);
        let tv4rhosigma30 = 2.0 * rho[ip] * t394 + 2.0 * t262;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t403 = piecewise3(t2, 0.0, t248 * t20 * t228 * t302 * t24 * t375 / 92160.0);
        let tv4sigma40 = 2.0 * rho[ip] * t403;
        v4sigma4[ip] += tv4sigma40;
    }
}
