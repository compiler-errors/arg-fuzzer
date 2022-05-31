
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9122(_: S7, _: S6, _: S4, _: S6, _: S5, _: S4, _: S5) {}
        
        fn test9122() { foo9122(S1, S6, S8, S4, S5, S2, S7, S3); }
    