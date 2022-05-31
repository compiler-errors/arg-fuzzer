
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5361(_: S2, _: S4) {}
        
        fn test5361() { foo5361(S1, S2, S3, S6, S7, S8); }
    