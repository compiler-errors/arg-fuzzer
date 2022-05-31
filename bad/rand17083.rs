
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo17083(_: S4, _: S7, _: S3) {}
        
        fn test17083() { foo17083(S4, S5, S8); }
    