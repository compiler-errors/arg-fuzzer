
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4527(_: S4, _: S6, _: S7) {}
        
        fn test4527() { foo4527(S3, S4, S2, S5, S3); }
    