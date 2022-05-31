
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5601(_: S5, _: S7) {}
        
        fn test5601() { foo5601(S2, S5, S2, S6, S3, S5, S2); }
    