
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3610(_: S2, _: S3, _: S8) {}
        
        fn test3610() { foo3610(S2, S6, S5, S3, S7, S7); }
    