
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5527(_: S2, _: S6) {}
        
        fn test5527() { foo5527(S4, S7, S5, S7, S3, S2, S5); }
    