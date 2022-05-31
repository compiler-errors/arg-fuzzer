
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3949(_: S6, _: S2, _: S7, _: S5, _: S7, _: S5) {}
        
        fn test3949() { foo3949(S8, S5, S2, S4, S7, S3, S1, S6); }
    