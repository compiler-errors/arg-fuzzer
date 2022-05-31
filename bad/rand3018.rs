
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3018(_: S4, _: S5, _: S3, _: S8, _: S7) {}
        
        fn test3018() { foo3018(S6, S5, S4, S2, S7, S1, S7); }
    