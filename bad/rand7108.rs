
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo7108(_: S5, _: S8) {}
        
        fn test7108() { foo7108(S2, S3, S4, S5, S7); }
    