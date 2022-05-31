
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5463(_: S7, _: S2, _: S6) {}
        
        fn test5463() { foo5463(S4, S3, S8, S7, S2, S1, S5); }
    