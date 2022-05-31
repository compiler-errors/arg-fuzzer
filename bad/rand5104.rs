
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5104(_: S6, _: S7, _: S8) {}
        
        fn test5104() { foo5104(S7, S3, S1, S5, S6, S2, S4); }
    