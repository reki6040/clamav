/*
 *  Copyright (C) 2013-2021 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
 *  Copyright (C) 2007-2013 Sourcefire, Inc.
 *
 *  Authors: Tomasz Kojm
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License version 2 as
 *  published by the Free Software Foundation.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
 *  MA 02110-1301, USA.
 */

#ifndef __SCANNERS_H
#define __SCANNERS_H

#include "clamav.h"
#include "others.h"
#include "filetypes.h"

/**
 * @brief Perform a magic scan of a file given a file descriptor.
 *
 * This API allows you to specify the file type in advance if you know it.
 *
 * @param desc      File descriptor
 * @param filepath  (optional) Full file path.
 * @param ctx       Scanning context structure.
 * @param type      CL_TYPE of data to be scanned.
 * @param name      (optional) Original name of the file (to set fmap name metadata)
 * @return cl_error_t
 */
cl_error_t cli_magic_scan_desc_type(int desc, const char *filepath, cli_ctx *ctx, cli_file_t type, const char *name);

/**
 * @brief Scan a tempfile / sub-file of _any_ type, passing in the fd, filepath (if available), and the scanning context.
 *
 * @param desc      File descriptor
 * @param filepath  (optional) Full file path.
 * @param ctx       Scanning context structure.
 * @param name      (optional) Original name of the file (to set fmap name metadata)
 * @return int      CL_SUCCESS, or an error code.
 */
cl_error_t cli_magic_scan_desc(int desc, const char *filepath, cli_ctx *ctx, const char *name);

/**
 * @brief Perform a magic scan on the current ctx.
 *
 * @param ctx       Scanning context structure.
 * @param type      CL_TYPE of data to be scanned.
 * @return int      CL_SUCCESS, or an error code.
 */
cl_error_t cli_magic_scan(cli_ctx *ctx, cli_file_t type);

/**
 * @brief   Scan an offset/length into a file map.
 *
 * Useful for scanning files or other type-able data embedded plainly in an existing fmap.
 *
 * Makes use of magic_scan_nested_fmap_type() for map scans when not forced to disk,
 * or if force-to-disk IS enabled, it will write the file to a temp file and then
 * will scan with cli_magic_scan_desc_type().
 *
 * @param map       File map.
 * @param offset    Offset into file map.
 * @param length    Length from offset.
 * @param ctx       Scanning context structure.
 * @param type      CL_TYPE of data to be scanned.
 * @param name      (optional) Original name of the file (to set fmap name metadata)
 * @return int      CL_SUCCESS, or an error code.
 */
cl_error_t cli_magic_scan_nested_fmap_type(cl_fmap_t *map, size_t offset, size_t length, cli_ctx *ctx, cli_file_t type, const char *name);

/**
 * @brief   Convenience wrapper for cli_magic_scan_nested_fmap_type().
 *
 * Creates an fmap and calls cli_magic_scan_nested_fmap_type() for you, with type CL_TYPE_ANY.
 *
 * @param buffer    Pointer to the buffer to be scanned.
 * @param length    Size in bytes of the buffer being scanned.
 * @param ctx       Scanning context structure.
 * @param name      (optional) Original name of the file (to set fmap name metadata)
 * @return int      CL_SUCCESS, or an error code.
 */
cl_error_t cli_magic_scan_buff(const void *buffer, size_t length, cli_ctx *ctx, const char *name);

cl_error_t cli_found_possibly_unwanted(cli_ctx *ctx);

/**
 * @brief   Internal-use version of cl_scanfile.
 *
 * This function will do a magic scan of an extracted file, given the file path.
 *
 * @param filename      Filepath of the file to be scanned.
 * @param ctx           Scanning context structure.
 * @param original_name (optional) Original name of the file (to set fmap name metadata)
 * @return cl_error_t
 */
cl_error_t cli_magic_scan_file(const char *filename, cli_ctx *ctx, const char *original_name);

/**
 * @brief   Internal function to recursively scan a directory.
 *
 * This function will do a magic scan of each file in a directory, given the path.
 *
 * @param dirname       Filepath of the directory to be scanned.
 * @param ctx           Scanning context structure.
 * @return cl_error_t
 */
cl_error_t cli_magic_scan_dir(const char *dirname, cli_ctx *ctx);

#endif
