
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9008(_: S2, _: S4, _: S5, _: S7) {}
        
        fn test9008() { foo9008(S1, S2, S4, S5, S6, S8); }
    